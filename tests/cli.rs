use std::fs::File;
use std::io::Write;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use assert_fs::prelude::*;

// main use case: loosen and repackage in order to add a snapshot to an existing pack. Should be
// faster than unpacking each single snapshot.
#[test]
fn run_loosen_and_repack() -> Result<(), Box<dyn std::error::Error>> {

    let temp = assert_fs::TempDir::new().unwrap();

    // 1. prepare: file foo.txt
    let input_file = temp.child("foo.txt");
    input_file.touch().unwrap();
    let mut file = File::open(input_file).unwrap();
    let _ = file.write("Snapshot 1 contents".as_ref());

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
    let input_file = temp.child("bar.txt");
    input_file.touch().unwrap();
    let mut file = File::open(input_file).unwrap();
    let _ = file.write("Snapshot 2 contents".as_ref());

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