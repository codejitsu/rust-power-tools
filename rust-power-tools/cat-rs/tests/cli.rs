use std::fs;

use assert_cmd::Command;
use rand::{distributions::Alphanumeric, Rng};

const PRG: &str = "cat-rs";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";
const BUSTLE: &str = "tests/inputs/the-bustle.txt";

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn gen_bad_file() -> String {
    loop {
        let filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

fn run_stdin(input_file: &str, args: &[&str], expected_file: &str) -> TestResult {
    let input = fs::read_to_string(input_file)?;
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn skips_bad_file() -> TestResult {
    let bad_file = gen_bad_file();
    let expected = format!("Failed to open {} (No such file or directory (os error 2))\n", bad_file);

    Command::cargo_bin(PRG)?
        .arg(&bad_file)
        .assert()
        .success()
        .stderr(expected);

    Ok(())
}

#[test]
fn test_bustle() -> TestResult {
    run(&[BUSTLE], "tests/expected/the-bustle.txt.out")
}

#[test]
fn test_empty() -> TestResult {
    run(&[EMPTY], "tests/expected/empty.txt.out")
}

#[test]
fn test_fox() -> TestResult {
    run(&[FOX], "tests/expected/fox.txt.out")
}

#[test]
fn test_spiders() -> TestResult {
    run(&[SPIDERS], "tests/expected/spiders.txt.out")
}

#[test]
fn test_bustle_numbers() -> TestResult {
    run(&[BUSTLE, "-n"], "tests/expected/the-bustle.numbers.txt.out")
}

#[test]
fn test_empty_numbers() -> TestResult {
    run(&[EMPTY, "-n"], "tests/expected/empty.numbers.txt.out")
}

#[test]
fn test_fox_numbers() -> TestResult {
    run(&[FOX, "-n"], "tests/expected/fox.numbers.txt.out")
}

#[test]
fn test_spiders_numbers() -> TestResult {
    run(&[SPIDERS, "-n"], "tests/expected/spiders.numbers.txt.out")
}

#[test]
fn test_bustle_blanks() -> TestResult {
    run(&[BUSTLE, "-b"], "tests/expected/the-bustle.blanks.txt.out")
}

#[test]
fn test_empty_blanks() -> TestResult {
    run(&[EMPTY, "-b"], "tests/expected/empty.blanks.txt.out")
}

#[test]
fn test_fox_blanks() -> TestResult {
    run(&[FOX, "-b"], "tests/expected/fox.blanks.txt.out")
}

#[test]
fn test_spiders_blanks() -> TestResult {
    run(&[SPIDERS, "-b"], "tests/expected/spiders.blanks.txt.out")
}

#[test]
fn with_stdin() -> TestResult {
    run_stdin("tests/inputs/fox.txt", &["-b"], "tests/expected/fox.blanks.txt.out")
}