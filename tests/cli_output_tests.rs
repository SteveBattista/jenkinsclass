use std::process::Command;

#[test]
fn binary_output_contains_header_recipient_and_quoted_fortune() {
    let output = Command::new(env!("CARGO_BIN_EXE_fortune"))
        .args(["--name", "Taylor", "--seed", "7"])
        .output()
        .expect("binary should execute");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be valid utf-8");
    assert!(stdout.contains("Fortune Cookie"));
    assert!(stdout.contains("This fortune is for Taylor."));
    assert!(stdout.contains('"'));
}

#[test]
fn list_flag_prints_multiple_fortunes() {
    let output = Command::new(env!("CARGO_BIN_EXE_fortune"))
        .arg("--list")
        .output()
        .expect("binary should execute");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be valid utf-8");
    assert!(stdout.lines().count() > 1);
    assert!(stdout.contains("Why do programmers prefer dark mode? Because light attracts bugs!"));
}
