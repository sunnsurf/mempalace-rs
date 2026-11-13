use std::process::Command;

#[test]
fn test_help_command() {
    // Cargo builds the package's binaries before its integration tests and
    // gives each test its path, wherever the toolchain lays out the target
    // directory.
    let output = Command::new(env!("CARGO_BIN_EXE_mempalace-rs"))
        .arg("--help")
        .output()
        .expect("failed to execute process");
    assert!(output.status.success());
    let help_text = String::from_utf8_lossy(&output.stdout);
    assert!(help_text.contains("Usage: mempalace-rs"));
}
