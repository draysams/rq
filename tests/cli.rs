use std::process::Command;

#[test]
fn echoes_the_arguments() {
    let output = Command::new(env!("CARGO_BIN_EXE_rq"))
        .arg("hello")
        .output()
        .expect("failed to run binary");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("You said hello"));
}