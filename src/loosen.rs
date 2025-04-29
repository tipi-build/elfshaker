use std::error::Error;

use clap::{App, Arg, ArgMatches};
use log::info;

use crate::packidx::FileEntry;
use crate::repo::{ExtractOptions, PackId};

use crate::utils::open_repo_from_cwd;

pub const SUBCOMMAND: &str = "loosen";

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let is_verify = matches.is_present("verify");
    let is_force = matches.is_present("force");

    // Parse --threads
    let threads: u32 = match matches.value_of("threads").unwrap().parse()? {
        0 => {
            let phys_cores = num_cpus::get_physical();
            info!(
                "-T|--threads=0: defaulting to number of physical cores (OS reports {} cores)",
                phys_cores
            );
            phys_cores as u32
        }
        n => n,
    };

    let mut opts = ExtractOptions::default();
    opts.set_verify(is_verify);
    // opts.set_reset(is_reset);
    opts.set_force(is_force);
    opts.set_num_workers(threads);

    let repo = open_repo_from_cwd()?;

    let pack_id = matches.value_of("pack").unwrap();

    let pack_id = PackId::Pack(pack_id.to_owned());
    let source_index = repo.load_index(&pack_id)?;

    let mut entries: Vec<FileEntry> = vec![];

    for snapshot in repo.load_index_snapshots(&pack_id)? {
        let mut _entries = source_index
            .resolve_snapshot(&snapshot)
            .expect("failed to resolve snapshot"); // TODO: Temporary.
        entries.append(&mut source_index.entries_from_handles(_entries.iter()).unwrap());
    }

    for entry in entries {
        repo.rewrite_loose_object(entry.path, &entry.checksum)?;
    }

    let snapshot_count = repo.load_index_snapshots(&pack_id).unwrap().len();
    eprintln!("Loosened {} snapshot(s)", snapshot_count);

    Ok(())
}

pub fn get_app() -> App<'static, 'static> {
    App::new(SUBCOMMAND)
        .about("Can be used to loosen a pack.")
        .arg(
            Arg::with_name("pack")
                .required(true)
                .index(1)
                .help("The tag of the pack to be loosened."),
        )
        .arg(
            Arg::with_name("verify")
                .long("verify")
                .help("Enables SHA-1 verification of the extracted files. This has a small performance overhead."),
        )
        .arg(Arg::with_name("force")
                .long("force")
                .help("Disables certain runtime checks that aim to detect unexpected file modification and prevent data loss."))
        .arg(Arg::with_name("threads")
                .short("T")
                .long("threads")
                .takes_value(true)
                .help("Use the specified number of worker threads for decompression. \
                      The number of threads used is proportional to the memory needed for decompression.")
                .default_value("0"))
}
