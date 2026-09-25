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
