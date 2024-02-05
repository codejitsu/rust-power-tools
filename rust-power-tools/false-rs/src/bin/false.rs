fn main() {
    std::process::exit(1);
}

#[test]
fn false_not_ok() {
    use assert_cmd::Command;
    
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}