fn main() {
    std::process::exit(0);
}

#[test]
fn true_ok() {
    use assert_cmd::Command;
    
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}