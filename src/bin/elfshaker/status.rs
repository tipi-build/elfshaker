//! SPDX-License-Identifier: Apache-2.0
//! Copyright (C) 2023 Tipi technologies or its affiliates and Contributors. All rights reserved.

use clap::{App, Arg, ArgMatches};
use crypto::{digest::Digest, sha1::Sha1};
use elfshaker::repo::{Repository, SnapshotId};
//use rand::RngCore;
use std::{error::Error as StdError, fs::File, io::Read, path::Path};

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
            true
        } else {
            let workspace_checksum = calculate_sha1(&path)?; // [0u8; 20];
            if entry.checksum != workspace_checksum {
                true
            } else {
                false
            }
        };
        //println!("{}", hex::encode(entry.checksum).to_string());
        if changed {
            changed_files.push(path.display().to_string());
        }
    }

    changed_files.sort();

    Ok(changed_files)
}

fn calculate_sha1(path: &Path) -> std::io::Result<[u8; 20]> {
    let mut file = File::open(path)?;
    let size = file.metadata().map(|m| m.len() as usize).ok();
    let mut buffer = Vec::with_capacity(size.unwrap_or(0));

    file.read_to_end(&mut buffer)?;

    let mut checksum = [0u8; 20];

    let mut hasher = Sha1::new();
    hasher.input(&buffer);
    hasher.result(&mut checksum);

    Ok(checksum)
}
