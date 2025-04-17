//! SPDX-License-Identifier: Apache-2.0
//! Copyright (C) 2021 Arm Limited or its affiliates and Contributors. All rights reserved.

pub mod atomicfile;
pub mod batch;
pub mod entrypool;
pub mod log;
pub mod packidx;
pub mod progress;
pub mod repo;

pub mod clone;
pub mod extract;
pub mod find;
pub mod list;
pub mod loosen;
pub mod pack;
pub mod show;
pub mod status;
pub mod store;
pub mod update;
pub mod utils;

use cxx::{CxxString, CxxVector};
use repo::ExtractResult;
use std::error::Error;
use std::path::PathBuf;
use std::string::String;

#[cxx::bridge(namespace = "elfshaker")]
mod bridge {

    extern "C++" {
        include!("elfshaker-cxxbridge/repo/repository.h");
        type ExtractOptions = crate::repo::repo_bridge::ExtractOptions;
        type ExtractResult = crate::repo::repo_bridge::ExtractResult;
    }

    extern "Rust" {
        /**
         * @brief Initialize the elfshaker_datadir structure so we can populate it with existing snapshots/packs later
         *
         * @param elfshaker_repo_dir
         * @return true
         * @return false
         */
        fn init_elfshaker_store(
            elfshaker_repo_dir: &CxxString,
            worktree_dir: &CxxString,
        ) -> Result<()>;

        /**
         * @brief Extract a pack or pack:snapshot
         *
         * @param elfshaker_repo_dir
         * @param pack
         * @param snapshot
         * @return whether the extraction was successful
         */
        fn extract(
            elfshaker_repo_dir: &CxxString,
            worktree_dir: &CxxString,
            snapshot: &CxxString,
            opts: ExtractOptions,
        ) -> Result<ExtractResult>;

        /**
         * @brief Create a snapshot in the specified repo given a list of files (paths must be relative to worktree!)
         *
         * @param elfshaker_repo_dir The parent folder of elfshaker_data/
         * @param files_to_snapshot
         * @param snapshot_name
         * @return true
         * @return false
         */
        fn store(
            elfshaker_repo_dir: &CxxString,
            worktree_dir: &CxxString,
            files_to_snapshot: &CxxVector<CxxString>,
            snapshot_name: &CxxString,
        ) -> Result<()>;

        /**
         * @brief Selectively pack a number of index files into a elfshaker pack
         *
         * @param elfshaker_repo_dir The parent folder of elfshaker_data/
         * @param pack_name the resulting pack name
         * @param index_files the list of files to include if empty will include all loose
         * @return true on success
         * @return false otherwise
         */
        fn pack(
            elfshaker_repo_dir: &CxxString,
            worktree_dir: &CxxString,
            pack_name: &CxxString,
            threads: u32,
            frames: u32,
        ) -> Result<()>;

        /**
         * \brief This computes the difference between stored files and files on disk for build pack snapshot
         * \param elfshaker_repo_dir
         * \param pack_snapshot_to_check_status_against
         */
        fn status(
            elfshaker_repo_dir: &CxxString,
            worktree_dir: &CxxString,
            pack_snapshot_to_check_status_against: &CxxString,
        ) -> Result<Vec<String>>;
    }
}

fn init_elfshaker_store(
    elfshaker_repo_dir: &CxxString,
    worktree_dir: &CxxString,
) -> Result<(), Box<dyn Error>> {
    let result = store::do_store(
        PathBuf::from(elfshaker_repo_dir.to_string()),
        PathBuf::from(worktree_dir.to_string()),
        "init",
        &Vec::<std::path::PathBuf>::new(),
    );
    result
}

fn extract(
    elfshaker_repo_dir: &CxxString,
    worktree_dir: &CxxString,
    snapshot: &CxxString,
    opts: bridge::ExtractOptions,
) -> Result<ExtractResult, Box<dyn Error>> {
    println!(
        "Options extracted {} {} {} {}",
        opts.verify(),
        opts.force(),
        opts.reset,
        opts.num_workers
    );
    let result = extract::do_extract(
        std::path::PathBuf::from(elfshaker_repo_dir.to_string()),
        std::path::PathBuf::from(worktree_dir.to_string()),
        &snapshot.to_string(),
        opts,
    )?;
    Ok(result)
}

fn store(
    elfshaker_repo_dir: &CxxString,
    worktree_dir: &CxxString,
    files_to_snapshot: &CxxVector<CxxString>,
    snapshot_name: &CxxString,
) -> Result<(), Box<dyn Error>> {
    let files_to_snapshot_paths: Vec<PathBuf> = files_to_snapshot
        .iter()
        .map(|s| PathBuf::from(s.to_string()).to_owned())
        .collect();

    let result = store::do_store(
        PathBuf::from(elfshaker_repo_dir.to_string()),
        PathBuf::from(worktree_dir.to_string()),
        &snapshot_name.to_str()?,
        &files_to_snapshot_paths,
    );

    result
}

const PACK_COMPRESSION_LEVEL: i32 = 10;

fn pack(
    elfshaker_repo_dir: &CxxString,
    worktree_dir: &CxxString,
    pack_name: &CxxString,
    threads: u32,
    frames: u32,
) -> Result<(), Box<dyn Error>> {
    pack::do_pack(
        std::path::PathBuf::from(elfshaker_repo_dir.to_string()),
        std::path::PathBuf::from(worktree_dir.to_string()),
        &pack_name.to_str()?,
        PACK_COMPRESSION_LEVEL,
        threads,
        frames,
        None,
    )
}

fn status(
    elfshaker_repo_dir: &CxxString,
    worktree_dir: &CxxString,
    pack_snapshot_to_check_status_against: &CxxString,
) -> Result<Vec<String>, Box<dyn Error>> {
    status::do_status(
        std::path::PathBuf::from(elfshaker_repo_dir.to_string()),
        std::path::PathBuf::from(worktree_dir.to_string()),
        &pack_snapshot_to_check_status_against.to_str()?,
    )
}
