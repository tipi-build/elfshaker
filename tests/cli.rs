use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::fs::File;
use std::io::Write;
use std::process::Command;

// main use case: loosen and repackage in order to add a snapshot to an existing pack. Should be
// faster than unpacking each single snapshot.
#[test]
fn run_loosen_and_repack() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new().unwrap();

    // 1. prepare: file foo.txt
    let foo_file = temp.child("foo.txt");
    println!("testing with {:#?}", foo_file.display());
    foo_file.touch().unwrap();

    foo_file
        .write_str("Snapshot 1 contents")
        .expect("unable to initialise foo.txt");

    // 2. prepare: create first snapshot
    let mut cmd = Command::cargo_bin("elfshaker")?;
    cmd.current_dir(temp.path());
    cmd.arg("store").arg("snapshot1");
    cmd.assert().success();

    // 3. prepare: pack first snapshot
    let mut cmd = Command::cargo_bin("elfshaker")?;
    cmd.current_dir(temp.path());
    cmd.arg("pack").arg("pack1");
    cmd.assert().success();

    // preparation done. we have a pack (same situation as downloaded pack with index)

    // 4. local state changes
    let bar_file = temp.child("bar.txt");
    bar_file.touch().unwrap();
    bar_file
        .write_str("Snapshot 2 contents")
        .expect("unable to write");

    // 5. loosen prepared/downloaded pack
    let mut cmd = Command::cargo_bin("elfshaker")?;
    cmd.current_dir(temp.path());
    cmd.arg("loosen").arg("pack1");
    cmd.assert().success();

    // 6. store local changes
    let mut cmd = Command::cargo_bin("elfshaker")?;
    cmd.current_dir(temp.path());
    cmd.arg("store").arg("snapshot2");
    cmd.assert().success();

    // 7. "repack"
    let mut cmd = Command::cargo_bin("elfshaker")?;
    cmd.current_dir(temp.path());
    cmd.arg("pack").arg("pack2");
    cmd.assert().success();

    // 8. check snapshots
    let mut cmd = Command::cargo_bin("elfshaker")?;
    cmd.current_dir(temp.path());
    cmd.arg("list").arg("pack2");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("snapshot1"))
        .stdout(predicate::str::contains("snapshot2"));
    Ok(())
}
