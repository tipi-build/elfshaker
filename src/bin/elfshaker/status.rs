//! SPDX-License-Identifier: Apache-2.0
//! Copyright (C) 2023 Tipi technologies or its affiliates and Contributors. All rights reserved.

use clap::{App, Arg, ArgMatches};
use crypto::{digest::Digest, sha1::Sha1};
use elfshaker::repo::{
    fs::{get_last_modified, open_file},
    Repository, SnapshotId,
};
use filetime::FileTime;
use log::info;
use std::{
    collections::HashSet,
    error::Error as StdError,
    fs::{self},
    io::{BufReader, Read},
    path::Path,
    sync::mpsc::channel,
};

use super::utils::{create_percentage_print_reporter, open_repo_from_cwd};

pub(crate) const SUBCOMMAND: &str = "status";

pub(crate) fn run(matches: &ArgMatches) -> Result<(), Box<dyn StdError>> {
    let json_output = matches.is_present("json_output");
    let snapshot_or_pack = matches
        .value_of("snapshot_or_pack")
        .expect("required argument");
    //let pack_id = PackId::from_str(snapshot_or_pack).expect("unable to parse snapshot_or_pack");
    let mut repo = open_repo_from_cwd()?;

    repo.set_progress_reporter(|msg| create_percentage_print_reporter(msg, 5));

    let changed_files: Vec<String> = if let Some(pack_id) = repo.is_pack(snapshot_or_pack)? {
        let index = repo.load_index_snapshots(&pack_id)?;
        println!("Pick one of these snapshots inside this pack: {:#?}", index);
        std::process::exit(23)
    } else {
        let snapshot = repo.find_snapshot(snapshot_or_pack)?;
        probe_snapshot_files(&repo, &snapshot)?
    };

    if json_output {
        let json = serde_json::to_string(&changed_files)
            .expect("internal structure will always serialize");
        println!("{json}");
    } else {
        println!("Compared to {snapshot_or_pack}");
        if changed_files.is_empty() {
            println!("working tree clean");
        } else {
            println!("\nChanged files:");
            for file in changed_files {
                println!("        {file}");
            }
            // Platform independent newline
            println!();
            // This error message is to harsh
            // return Err(Box::new(Error::DirtyWorkDir));
        }
    }

    Ok(())
}

pub(crate) fn get_app() -> App<'static, 'static> {
    App::new(SUBCOMMAND)
        .about("Displays the difference between the current directory and a stored snapshot.")
        .arg(
            Arg::with_name("snapshot_or_pack")
                .required(true)
                .index(1)
                .help("The tag of the existing snapshot or pack."),
        )
        .arg(
            Arg::with_name("json_output")
                .long("json")
                .required(false)
                .help("Output the result as a JSON"),
        )
}

fn probe_snapshot_files(
    repo: &Repository,
    snapshot: &SnapshotId,
) -> Result<Vec<String>, Box<dyn StdError>> {
    let pool = threadpool::ThreadPool::new(1);
    let (workspace_files_sender, workspace_files_receiver) = channel();
    pool.execute(move || {
        let base_dir = std::env::current_dir().expect("unable to get current working directory");
        let mut normalised_paths = HashSet::new();
        let mut symlink_targets_in_tree = HashSet::new();

        let walker = walkdir::WalkDir::new(".");
        for entry in walker {
            let entry = entry.unwrap();
            let metadata = entry.metadata().expect("unable to stat metadata");

            if metadata.is_dir() {
                continue;
            }
            let original_path = entry.path().display().to_string();

            #[cfg(target_family = "windows")]
            let path = Repository::replace_back_to_slash(&original_path);
            #[cfg(not(target_family = "windows"))]
            let path = original_path.clone();

            if path != "." && !path.starts_with("./elfshaker_data") {
                normalised_paths.insert(path);

                if metadata.is_symlink() {
                    if let Ok(target) = fs::read_link(original_path) {
                        let target = if target.is_absolute() {
                            // make relative
                            if let Ok(target) = target.strip_prefix(&base_dir) {
                                target
                            } else {
                                // out of tree, skipping
                                continue;
                            }
                        } else {
                            &target
                        };

                        let path = target.display().to_string();
                        #[cfg(target_family = "windows")]
                        let path = Repository::replace_back_to_slash(&*path);

                        //println!("symlink target: {path}");
                        symlink_targets_in_tree.insert(path);
                    }
                }
            }
        }

        let filtered_paths = normalised_paths
            .difference(&symlink_targets_in_tree)
            .cloned()
            .collect();

        workspace_files_sender
            .send(filtered_paths)
            .expect("unable to send file list to main thread");
    });

    let mut changed_files = HashSet::new(); // vec![];
    let mut unchanged_files = HashSet::new(); // vec![];

    let idx = repo.load_index(snapshot.pack())?;
    let handles = idx
        .resolve_snapshot(snapshot.tag())
        .expect("failed to resolve snapshot"); // TODO: Temporary.

    for entry in idx.entries_from_handles(handles.iter())? {
        let path = Path::new(&entry.path);

        let changed = if path.exists() == false {
            // missing in workspace
            info!("not in workspace {}", path.display());
            true
        } else {
            let file_in_workspace_is_symlink = path.is_symlink();

            if entry.file_metadata.is_symlink_file != file_in_workspace_is_symlink {
                info!(
                    "symlink status differs from recording for {}",
                    path.display()
                );
                true
            } else if file_in_workspace_is_symlink {
                let workspace_target = fs::read_link(path)?;
                let changed = entry.file_metadata.symlink_target != workspace_target;
                if changed {
                    info!(
                        "changed \"{}\": index.symlink: {}; fs.symlink: {};",
                        path.display(),
                        entry.file_metadata.symlink_target.display(),
                        workspace_target.display()
                    );
                    true
                } else {
                    info!(
                        "same \"{}\": symlink_target: {};",
                        path.display(),
                        workspace_target.display()
                    );
                    false
                }
            } else {
                let workspace_mtime = get_last_modified(path.metadata()?);
                if workspace_mtime.is_some()
                    && FileTime::from_unix_time(
                        entry.file_metadata.last_modified,
                        entry.file_metadata.last_modified_nanos,
                    ) == workspace_mtime.unwrap().into()
                {
                    info!("same mtime \"{}\"", path.display());
                    false
                } else {
                    let workspace_checksum = calculate_sha1(path)?;

                    if entry.checksum != workspace_checksum {
                        info!(
                            "changed \"{}\": index.checksum: {}; fs.checksum: {};",
                            path.display(),
                            hex::encode(entry.checksum),
                            hex::encode(workspace_checksum)
                        );
                        true
                    } else {
                        info!(
                            "same \"{}\": checksum: {};",
                            path.display(),
                            hex::encode(workspace_checksum)
                        );
                        false
                    }
                }
            }
        };

        let mut path_string = path.display().to_string();
        if path_string.starts_with("./") == false {
            path_string = format!("./{}", path_string);
        }

        if changed {
            changed_files.insert(path_string);
        } else {
            unchanged_files.insert(path_string);
        }
    }

    let workspace_file_paths = workspace_files_receiver
        .recv()
        .expect("unable to fetch sorted file list from worker thread");

    Ok(add_untracked_files(
        changed_files,
        unchanged_files,
        workspace_file_paths,
    ))
}

fn add_untracked_files(
    changed_files: HashSet<String>,
    unchanged_files: HashSet<String>,
    workspace_file_paths: HashSet<String>,
) -> Vec<String> {
    let any_changes = workspace_file_paths
        .difference(&unchanged_files)
        .cloned()
        .collect::<HashSet<_>>();

    let all_changes = changed_files.union(&any_changes);

    let mut list = all_changes.cloned().collect::<Vec<String>>();
    list.sort();

    list
}

fn calculate_sha1(path: &Path) -> std::io::Result<[u8; 20]> {
    let file = open_file(path)?;
    let mut file_handler = BufReader::new(file);

    let mut checksum = [0u8; 20];

    let mut hasher = Sha1::new();

    let mut buffer = vec![0u8; 4096];
    while let Ok(bytes_read) = file_handler.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        hasher.input(&buffer[..bytes_read]);
    }

    hasher.result(&mut checksum);

    Ok(checksum)
}

#[cfg(test)]
mod test {
    use super::*;

    fn vs(list: Vec<&str>) -> Vec<String> {
        list.into_iter().map(str::to_string).collect()
    }
    fn hs(list: Vec<&str>) -> HashSet<String> {
        list.into_iter().map(str::to_string).collect()
    }

    #[test]
    fn add_front() {
        let e = vs(vec!["a", "b", "c"]);
        let r = add_untracked_files(hs(vec!["b", "c"]), HashSet::new(), hs(vec!["a", "b", "c"]));
        assert_eq!(e, r);
    }

    #[test]
    fn add_middle() {
        let e = vs(vec!["a", "b", "c"]);
        let r = add_untracked_files(hs(vec!["b", "c"]), HashSet::new(), hs(vec!["a", "b", "c"]));
        assert_eq!(e, r);
    }

    #[test]
    fn add_back() {
        let e = vs(vec!["a", "b", "c"]);
        let r = add_untracked_files(hs(vec!["b", "c"]), HashSet::new(), hs(vec!["a", "b", "c"]));
        assert_eq!(e, r);
    }
    #[test]
    fn no_middle() {
        let e = vs(vec!["a", "c"]);
        let r = add_untracked_files(hs(vec!["c"]), hs(vec!["b"]), hs(vec!["a", "b", "c"]));
        assert_eq!(e, r);
    }
}
