use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_cat_t() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cat")?;
    cmd.arg("-t").arg("tests/data/input.txt");
    cmd.assert().success().stdout(predicate::str::contains("^I"));
    Ok(())
}

#[test]
fn test_cat_n() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cat")?;
    cmd.arg("-n").arg("tests/data/input.txt");
    cmd.assert().success().stdout(predicate::str::starts_with("     1\t"));
    Ok(())
}

#[test]
fn test_cat_multiple_files() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cat")?;
    cmd.arg("tests/data/input.txt").arg("tests/data/input2.txt");
    cmd.assert().success().stdout(predicate::str::contains("line 1\nline 2\nline 3\nline 1\nline 2\nline 3\n"));
    Ok(())
}

#[test]
fn test_cat_non_existent_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cat")?;
    cmd.arg("tests/data/non_existent.txt");
    cmd.assert().failure().stderr(predicate::str::contains("No such file or directory"));
    Ok(())
}
