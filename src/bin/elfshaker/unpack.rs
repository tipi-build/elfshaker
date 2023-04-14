use std::error::Error;

use clap::{App, Arg, ArgMatches};
use crate::utils::open_repo_from_cwd;

use elfshaker::repo::{LOOSE_DIR, PackId, Repository, SnapshotId};

pub(crate) const SUBCOMMAND: &str = "unpack";

pub(crate) fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    // let is_verify = matches.is_present("verify");
    // let is_force = matches.is_present("force");

    // Parse --threads
    // let threads: u32 = match matches.value_of("threads").unwrap().parse()? {
    //     0 => {
    //         let phys_cores = num_cpus::get_physical();
    //         info!(
    //             "-T|--threads=0: defaulting to number of physical cores (OS reports {} cores)",
    //             phys_cores
    //         );
    //         phys_cores as u32
    //     }
    //     n => n,
    // };

    // let new_head = match repo.find_snapshot(snapshot) {
    //     Err(RepoError::PackError(PackError::SnapshotNotFound(_))) => {
    //         info!("Snapshot not available locally. Updating remotes...");
    //         repo.update_remotes()?;
    //         repo.find_snapshot(snapshot)?
    //     }
    //     r => r?,
    // };



    let mut repo = open_repo_from_cwd()?;

    let pack_id = matches.value_of("pack").unwrap();
    // open pack
    let pack_id = PackId::Pack(pack_id.to_owned());
    let pack = repo.open_pack(&pack_id).unwrap();

    // TODO: get entries from index

    // TODO: maybe clear LOOSE_DIR or handle overwrite

    // let entries = None;
    repo.extract_entries(&pack_id, &[], LOOSE_DIR, Default::default()).expect("could not extract");

    // iterate through snapshots and extract each of them to the LOOSE_DIR
    // pack.extract_entries()


    // match repo.read_head()? {
    //     (Some(h), _) if h == new_head && !is_reset => {
    //         // The specified snapshot is already extracted and --reset is not specified,
    //         // so this is a no-op.
    //         warn!(
    //             "HEAD is already at {} and --reset is not specified. Exiting early...",
    //             h,
    //         );
    //         return Ok(());
    //     }
    //     _ => {}
    // };

    // let mut opts = ExtractOptions::default();
    // opts.set_verify(is_verify);
    // opts.set_reset(is_reset);
    // opts.set_force(is_force);
    // opts.set_num_workers(threads);
    //
    // repo.set_progress_reporter(|msg| create_percentage_print_reporter(msg, 5));
    // let result = repo.extract_snapshot(new_head.clone(), opts)?;
    //
    // eprintln!("A \t{} files", result.added_file_count);
    // eprintln!("D \t{} files", result.removed_file_count);
    // eprintln!("M \t{} files", result.modified_file_count);
    // eprintln!("Unpacked'{}'", new_head);
    eprintln!("Unpack was executed without doing anything");

    Ok(())
}

pub(crate) fn get_app() -> App<'static, 'static> {
    App::new(SUBCOMMAND)
        .about("Can be used to unpack a pack to a loose pack.")
        .arg(
            Arg::with_name("pack")
                .required(true)
                .index(1)
                .help("The tag of the pack to be unpacked."),
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
