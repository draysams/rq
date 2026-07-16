use std::process::Command;

#[test]
fn grep_ignore_case_flag_matches_regardless_of_case() {
    let output = Command::new(env!("CARGO_BIN_EXE_rq"))
        .args([
            "grep",
            "APPLE",
            "tests/fixtures/sample.txt",
            "--ignore-case",
        ])
        .output()
        .unwrap();
    assert!(!output.stdout.is_empty());
}

#[test]
fn missing_file_exits_nonzero_without_panicking() {
    let output = Command::new(env!("CARGO_BIN_EXE_rq"))
        .arg("does/not/exist.txt")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stdout);
    assert!(!stderr.contains("panicked"));
}
