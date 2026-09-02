use std::process::Command;

#[test]
fn ocr_smoke_help_succeeds() {
    let binary = match option_env!("CARGO_BIN_EXE_ocr_smoke") {
        Some(path) => path,
        None => {
            eprintln!("CARGO_BIN_EXE_ocr_smoke not set; skipping test");
            return;
        }
    };

    let output = Command::new(binary)
        .arg("--help")
        .output()
        .expect("failed to execute ocr_smoke --help");

    assert!(
        output.status.success(),
        "expected success status, got: {:?}",
        output.status
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Usage:"),
        "help output did not contain usage text: {}",
        stdout
    );
}
