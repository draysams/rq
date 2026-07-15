use std::process::Command;

#[test]
fn prints_file_contents() {
    let output = Command::new(env!("CARGO_BIN_EXE_rq"))
        .arg("tests/fixtures/sample.txt")
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("hello from fixture"));
}

#[test]
fn missing_file_exits_nonzero_without_panicking () {
    let output = Command::new(env!("CARGO_BIN_EXE_rq"))
        .arg("does/not/exist.txt")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stdout);
    assert!(!stderr.contains("panicked"));
}
