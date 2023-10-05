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
//use rand::RngCore;
use std::{
    error::Error as StdError,
    fs::{self, FileType},
    io::{BufReader, Read},
    path::Path,
    time::SystemTime,
};

use super::utils::{create_percentage_print_reporter, open_repo_from_cwd};
//use elfshaker::repo::fs::open_file;
//use elfshaker::repo::ExtractOptions;

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
        //println!("is pack: {:#?}", pack_id);
        // print_pack_summary(&repo, pack_id)?;

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
            println!("");
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
    let mut changed_files = vec![];

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
            let workspace_is_symlink = path.is_symlink();

            if entry.file_metadata.is_symlink_file != workspace_is_symlink {
                info!(
                    "symlink status differs from recording for {}",
                    path.display()
                );
                true
            } else {
                if workspace_is_symlink {
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
                        let workspace_checksum = calculate_sha1(&path)?;

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
            }
        };

        if changed {
            changed_files.push(path.display().to_string());
        }
    }

    changed_files.sort();

    Ok(changed_files)
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
