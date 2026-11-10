use std::process::Command;

#[test]
fn version_flags_report_the_cargo_package_version() {
    for flag in ["--version", "-V"] {
        let output = Command::new(env!("CARGO_BIN_EXE_jseries"))
            .arg(flag)
            .output()
            .unwrap();
        assert!(output.status.success(), "{flag} failed");
        assert!(output.stderr.is_empty(), "{flag} wrote to stderr");
        assert_eq!(
            String::from_utf8(output.stdout).unwrap().trim(),
            format!("jseries {}", env!("CARGO_PKG_VERSION"))
        );
    }
}

#[test]
fn decode_accepts_information_words_and_rejects_removed_format_option() {
    let schema = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../schemas");
    let args = [
        "decode",
        "--schema",
        schema.to_str().unwrap(),
        "--message",
        "EXAMPLE-70",
        "--word",
        "0x1c94",
    ];
    let output = Command::new(env!("CARGO_BIN_EXE_jseries"))
        .args(args)
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(
        String::from_utf8(output.stdout)
            .unwrap()
            .contains("message=EXAMPLE-70")
    );
    for format in ["logical70", "word75", "simple80"] {
        let output = Command::new(env!("CARGO_BIN_EXE_jseries"))
            .args(args)
            .args(["--input-format", format])
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2));
        assert!(
            String::from_utf8(output.stderr)
                .unwrap()
                .contains("unknown option --input-format")
        );
    }
}
