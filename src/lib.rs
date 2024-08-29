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

//use std::fmt::{self, Display};
//
//#[derive(Debug)]
//struct Error;
//
//impl std::error::Error for Error {}
//
//impl Display for Error {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        f.write_str("rust error")
//    }
//}

#[cxx::bridge(namespace = "bridge")]
mod bridge {

  extern "C++" {
    include!("elfshaker-cxxbridge/repo/repository.h");
    type ExtractOptions = crate::repo::repo_bridge::ExtractOptions;
    type ExtractResult = crate::repo::repo_bridge::ExtractResult;
  }

  extern "Rust" {
    fn print(slice: &[u64]);

    /**
     * @brief Initialize the elfshaker_datadir structure so we can populate it with existing snapshots/packs later
     * 
     * @param elfshaker_repo_dir 
     * @return true 
     * @return false 
     */
    fn init_elfshaker_store(elfshaker_repo_dir: &CxxString, worktree_dir: &CxxString) -> Result<()>;
    
    /**
     * @brief Extract a pack or pack:snapshot
     * 
     * @param elfshaker_repo_dir 
     * @param pack 
     * @param snapshot 
     * @return whether the extraction was successful
     */
    fn extract(elfshaker_repo_dir: &CxxString, worktree_dir: &CxxString, pack: &CxxString, snapshot: &CxxString, opts: ExtractOptions) -> Result<ExtractResult>;

    /**
     * @brief Create a snapshot in the specified repo given a list of files (paths must be relative to worktree!)
     * 
     * @param elfshaker_repo_dir The parent folder of elfshaker_data/
     * @param files_to_snapshot 
     * @param snapshot_name 
     * @return true 
     * @return false 
     */
    fn store(elfshaker_repo_dir: &CxxString, worktree_dir: &CxxString, files_to_snapshot: &CxxVector<CxxString>, snapshot_name: &CxxString) -> Result<()>;
  }  
} 

fn print(slice: &[u64]) {
    println!("Hello cxxbridge from foo/mod.rs! {:?}", slice);
}

fn init_elfshaker_store(elfshaker_repo_dir: &CxxString, worktree_dir: &CxxString) -> Result<(), Box<dyn Error>> {

  println!("Creating it in {:?}", elfshaker_repo_dir);
  
  //let arg_vec = vec!["store", "init", "--files-from", "-"];
  //let arg_matches = crate::store::get_app().get_matches_from(arg_vec);
  //store::run(&arg_matches);
  let result = store::do_store(
    PathBuf::from(elfshaker_repo_dir.to_string()),
    PathBuf::from(worktree_dir.to_string()),
    "init", &Vec::<std::path::PathBuf>::new());
  println!("Error : {:?}", result);
  result
  //Err("result".to_string())
  //return store(elfshaker_repo_dir, {}, "init");
}

fn extract(elfshaker_repo_dir: &CxxString, worktree_dir: &CxxString, pack: &CxxString, snapshot: &CxxString, opts: bridge::ExtractOptions) -> Result<ExtractResult, Box<dyn Error>> {
  println!("Options extracted {} {} {} {}", opts.verify(), opts.force(), opts.reset, opts.num_workers);
  let result = extract::do_extract(std::path::PathBuf::from(elfshaker_repo_dir.to_string()),std::path::PathBuf::from(worktree_dir.to_string()), &snapshot.to_string(), opts)?;
  Ok(result)

  //TODO: Add to repository constructor the option to specify elfshaker_repo_dir (data_dir) and the separate workdir
}

fn store(elfshaker_repo_dir: &CxxString, worktree_dir: &CxxString, files_to_snapshot: &CxxVector<CxxString>, snapshot_name: &CxxString) -> Result<(), Box<dyn Error>> {
  let files_to_snapshot_paths: Vec<PathBuf> = files_to_snapshot 
    .iter()
    .map(|s| PathBuf::from(s.to_string()).to_owned())
    .collect();


  let result = store::do_store(
    PathBuf::from(elfshaker_repo_dir.to_string()),
    PathBuf::from(worktree_dir.to_string()),
    &snapshot_name.to_str()?, &files_to_snapshot_paths);
  
  result
}