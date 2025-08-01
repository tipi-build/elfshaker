//! SPDX-License-Identifier: Apache-2.0
//! Copyright (C) 2022 Arm Limited or its affiliates and Contributors. All rights reserved.

use clap::{App, Arg, ArgMatches};
use rand::RngCore;
use std::error::Error;
use std::{
    fs,
    path::{Path, PathBuf},
};

use super::utils::create_percentage_print_reporter;
use crate::repo::{self, Repository};

pub const SUBCOMMAND: &str = "clone";

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let origin_url = matches.value_of("repository").unwrap();
    let directory = matches.value_of("directory").unwrap();

    let original_cwd = std::env::current_dir()?;
    if Path::new(directory).exists() {
        return Err(format!("'{}' already exists!", directory).into());
    }

    let temp_directory = PathBuf::from(format!(
        ".{}-{}",
        directory.to_owned(),
        create_random_name()
    ));
    if let Err(e) = do_clone(&temp_directory, origin_url) {
        let _ = fs::remove_dir_all(temp_directory);
        return Err(e);
    };
    fs::rename(
        original_cwd.join(temp_directory),
        original_cwd.join(directory),
    )?;

    Ok(())
}

fn do_clone(directory: &Path, origin_url: &str) -> Result<(), Box<dyn Error>> {
    fs::create_dir(directory)?;
    fs::create_dir(directory.join(repo::REPO_DIR))?;

    let mut repo = Repository::open(directory)?;

    repo.set_progress_reporter(|msg| create_percentage_print_reporter(msg, 5));
    repo.add_remote("origin", origin_url)?;
    repo.update_remotes()?;
    Ok(())
}

pub fn get_app() -> App<'static, 'static> {
    App::new(SUBCOMMAND)
        .about("Clones a remote repository into a new directory")
        .arg(
            Arg::with_name("repository")
                .required(true)
                .index(1)
                .help("The URL of the remote repository index (.esi) to clone."),
        )
        .arg(
            Arg::with_name("directory")
                .required(true)
                .index(2)
                .help("The name of a new directory to clone into."),
        )
}

fn create_random_name() -> String {
    let mut bytes = [0u8; 8];
    rand::thread_rng().fill_bytes(&mut bytes);
    hex::encode(bytes)
}
