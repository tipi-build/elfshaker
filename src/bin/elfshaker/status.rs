//! SPDX-License-Identifier: Apache-2.0
//! Copyright (C) 2023 Tipi technologies or its affiliates and Contributors. All rights reserved.

use clap::{App, Arg, ArgMatches};
use rand::RngCore;
use std::{collections::HashMap, error::Error};

use super::utils::{create_percentage_print_reporter, open_repo_from_cwd};
use elfshaker::repo::fs::open_file;
use elfshaker::repo::ExtractOptions;

pub(crate) const SUBCOMMAND: &str = "status";

pub(crate) fn run(_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let mut repo = open_repo_from_cwd()?;

    repo.set_progress_reporter(|msg| create_percentage_print_reporter(msg, 5));
    repo.update_remotes()?;

    Ok(())
}

pub(crate) fn get_app() -> App<'static, 'static> {
    App::new(SUBCOMMAND)
        .about("Displays the difference between the current directory and a stored snapshot.")
}
