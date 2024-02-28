use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echo-rs")?;

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("echo-rs")?;

    cmd.arg("hello").assert().success();
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    let mut cmd = Command::cargo_bin("echo-rs")?;

    cmd.arg("Hello there")
        .assert()
        .success()
        .stdout("Hello there\n\n");
    Ok(())
}

#[test]
fn hello2() -> TestResult {
    let mut cmd = Command::cargo_bin("echo-rs")?;

    cmd.args(vec!["Hello", "there"])
        .assert()
        .success()
        .stdout("Hello there\n\n");
    Ok(())
}

#[test]
fn hello3() -> TestResult {
    let mut cmd = Command::cargo_bin("echo-rs")?;

    cmd.args(vec!["     Hello ", "   there", "-n"])
        .assert()
        .success()
        .stdout("Hello there\n");
    Ok(())
}

#[test]
fn hello4() -> TestResult {
    let mut cmd = Command::cargo_bin("echo-rs")?;

    cmd.args(vec!["-n", "Hello", "there"])
        .assert()
        .success()
        .stdout("Hello there\n");
    Ok(())
}
