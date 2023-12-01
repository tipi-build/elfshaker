use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::{fs::remove_file, process::Command};

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

#[test]
fn package_file_see_status() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new().unwrap();

    // 1. prepare: file foo.txt
    let foo_file = temp.child("foo.txt");
    foo_file.touch().unwrap();
    foo_file
        .write_str("Snapshot 1 contents")
        .expect("unable to write initially");

    // 2. prepare: create first snapshot
    let mut cmd = Command::cargo_bin("elfshaker")?;
    cmd.args(["store", "snapshot1"]);
    cmd.current_dir(temp.path());
    cmd.assert().success();

    // 3. prepare: pack first snapshot
    let mut cmd = Command::cargo_bin("elfshaker")?;
    cmd.args(["pack", "pack1"]);
    cmd.current_dir(temp.path());
    cmd.assert().success();

    // 4. check status

    let mut cmd = Command::cargo_bin("elfshaker")?;
    cmd.args(["status", "--json", "pack1:snapshot1"]);
    cmd.current_dir(temp.path());
    let clean = cmd.output()?;
    assert!(clean.status.success());
    assert_eq!("[]\n".to_string(), String::from_utf8_lossy(&clean.stdout));
    //.assert().success();

    // 5. modify file
    println!("modify foo.txt");
    foo_file
        .write_str("Snapshot 1 contents appended")
        .expect("unable to update foo.txt");

    // 6. check status: ["foo.txt"]
    let mut cmd = Command::cargo_bin("elfshaker")?;
    cmd.args(["status", "--json", "pack1:snapshot1"]);
    cmd.current_dir(temp.path());
    let dirty = cmd.output()?;
    assert!(dirty.status.success());
    assert_eq!(
        "[\"foo.txt\"]\n".to_string(),
        String::from_utf8_lossy(&dirty.stdout)
    );

    Ok(())
}

#[test]
fn package_file_see_status_symlink() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new().unwrap();

    // 1. prepare: file foo.txt and snapshot
    let foo_file = temp.child("foo.txt");
    foo_file.touch().unwrap();
    foo_file
        .write_str("FOO FOO CONTENT")
        .expect("unable to write initially");

    let mut symlink_file = temp.to_path_buf();
    symlink_file.push("link.txt");

    cfg_if::cfg_if! {
        if #[cfg(target_family = "unix")] {
            use std::os::unix::fs::symlink;
        } else if #[cfg(target_family = "windows")] {
            use std::os::windows::fs::symlink_file as symlink;
        } else {
            compile_error!("symlink not implemented for target_family");
        }
    }
    symlink(foo_file.path(), &symlink_file).expect("unable to create symlink");

    // 2. prepare: create first snapshot
    let mut cmd = Command::cargo_bin("elfshaker")?;
    cmd.args(["store", "snapshot1"]);
    cmd.current_dir(temp.path());
    cmd.assert().success();

    // 3. prepare: pack first snapshot
    let mut cmd = Command::cargo_bin("elfshaker")?;
    cmd.args(["pack", "pack1"]);
    cmd.current_dir(temp.path());
    cmd.assert().success();

    // 4. check status

    let mut cmd = Command::cargo_bin("elfshaker")?;
    cmd.args(["status", "--json", "pack1:snapshot1"]);
    cmd.current_dir(temp.path());
    let clean = cmd.output()?;
    assert!(clean.status.success());
    assert_eq!("[]", String::from_utf8_lossy(&clean.stdout).trim());
    //.assert().success();

    // 5. modify file
    println!("modify foo.txt");
    foo_file
        .write_str("FOO File Replaced")
        .expect("unable to update foo.txt");

    // 6. check status: ["foo.txt"]
    let mut cmd = Command::cargo_bin("elfshaker")?;
    cmd.args(["status", "--json", "pack1:snapshot1"]);
    cmd.current_dir(temp.path());
    let dirty = cmd.output()?;
    assert!(dirty.status.success());
    assert_eq!(
        "[\"foo.txt\"]\n".to_string(),
        String::from_utf8_lossy(&dirty.stdout)
    );

    // 7. modify symlink
    let bar_file = temp.child("bar.txt");
    bar_file
        .write_str("bar bar")
        .expect("unable to write bar.txt");
    remove_file(&symlink_file).expect("unable to remove symlink_file");
    symlink(&bar_file, symlink_file).expect("unable to update symlink");

    // 8. check status:
    let mut cmd = Command::cargo_bin("elfshaker")?;
    cmd.args(["status", "--json", "pack1:snapshot1"]);
    cmd.current_dir(temp.path());
    let dirty = cmd.output()?;
    assert!(dirty.status.success());
    assert_eq!(
        r#"["bar.txt","foo.txt","link.txt"]"#.to_string(),
        String::from_utf8_lossy(&dirty.stdout).trim()
    );

    Ok(())
}

#[test]
fn symlink_to_directory() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new().unwrap();

    // 1. prepare: Folders and Files and Symlink
    let real_dir = temp.child("real_directory");
    real_dir
        .create_dir_all()
        .expect("unable to create real directory");

    let some_sub_file = real_dir.child("some.txt");
    some_sub_file
        .write_str("some data")
        .expect("unable to create some.txt");

    let real_dir_2 = temp.child("real_dir_2");
    real_dir_2.create_dir_all().expect("unable to create dir 2");

    let some_other_file = real_dir_2.child("other.txt");
    some_other_file
        .write_str("some other file content")
        .expect("unable to write other.txt");

    let mut symlink_dir = temp.to_path_buf();
    symlink_dir.push("link_dir");

    cfg_if::cfg_if! {
        if #[cfg(target_family = "unix")] {
            use std::os::unix::fs::symlink;
        } else if #[cfg(target_family = "windows")] {
            use std::os::windows::fs::symlink_dir as symlink;
        } else {
            compile_error!("symlink not implemented for target_family");
        }
    }
    symlink(real_dir.path(), &symlink_dir).expect("unable to create symlink");

    // 2. prepare: create first snapshot
    let mut cmd = Command::cargo_bin("elfshaker")?;
    cmd.args(["store", "snapshot1"]);
    cmd.current_dir(temp.path());
    cmd.assert().success();

    // 3. prepare: pack first snapshot
    let mut cmd = Command::cargo_bin("elfshaker")?;
    cmd.args(["pack", "pack1"]);
    cmd.current_dir(temp.path());
    cmd.assert().success();

    // 4. check status
    let mut cmd = Command::cargo_bin("elfshaker")?;
    cmd.args(["status", "--json", "pack1:snapshot1"]);
    cmd.current_dir(temp.path());
    let clean = cmd.output()?;
    assert!(clean.status.success());
    assert_eq!(
        r#"["link_dir"]"#,
        String::from_utf8_lossy(&clean.stdout).trim()
    );

    // 5. modify symlink
    println!("removing symlink_dir");
    cfg_if::cfg_if! {
        if #[cfg(target_family = "unix")] {
            remove_file(&symlink_dir).expect("unable to remove symlink_dir");
        } else if #[cfg(target_family = "windows")] {
            std::fs::remove_dir(&symlink_dir).expect("unable to remove symlink_dir");
        } else {
            compile_error!("symlink not implemented for target_family");
        }
    }
    println!("creating updated symlink_dir");
    symlink(&real_dir_2, symlink_dir).expect("unable to update symlink");

    // 6. check status:
    println!("final status");
    let mut cmd = Command::cargo_bin("elfshaker")?;
    cmd.args(["status", "--json", "pack1:snapshot1"]);
    cmd.current_dir(temp.path());
    let dirty = cmd.output()?;
    assert!(dirty.status.success());
    // old some.txt is stored by elfshaker twice
    assert_eq!(
        r#"["link_dir","link_dir/some.txt"]"#.to_string(),
        String::from_utf8_lossy(&dirty.stdout).trim()
    );

    Ok(())
}
