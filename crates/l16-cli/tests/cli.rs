use std::process::{Command,Output};
fn run(args:&[&str])->Output {
    Command::new(env!("CARGO_BIN_EXE_l16-inspect")).args(args).output().unwrap()
}
#[test]
fn req_cli_001_no_implicit_baseline() {
    let o=run(&["demo","--word","0x0765","--evidence","test"]);
    assert_eq!(o.status.code(),Some(2));
    assert!(String::from_utf8_lossy(&o.stderr).contains("missing --baseline"));
}
#[test]
fn req_cli_001_no_implicit_selection_evidence() {
    let o=run(&["demo","--baseline","D","--word","0x0765"]);
    assert_eq!(o.status.code(),Some(2));
}
#[test]
fn req_cli_002_demo_is_clearly_marked() {
    let o=run(&["demo","--baseline","H","--word","0x0765","--evidence","test"]);
    assert!(o.status.success());
    let text=String::from_utf8_lossy(&o.stdout);
    assert!(text.contains("SYNTHETIC ONLY"));
    assert!(text.contains("numerator: 250"));
    assert!(text.contains("schema_sha256="));
}
#[test]
fn req_cli_003_standards_mode_never_uses_demo_schema() {
    let o=run(&["decode","--baseline","H"]);
    assert_eq!(o.status.code(),Some(3));
    assert!(o.stdout.is_empty());
    assert!(String::from_utf8_lossy(&o.stderr).contains("no verified MIL-STD-6016 schema"));
}
#[test]
fn req_cli_004_duplicate_option_is_rejected() {
    let o=run(&["decode","--baseline","D","--baseline","H"]);
    assert_eq!(o.status.code(),Some(2));
}
#[test]
fn req_cli_004_unknown_option_is_rejected() {
    let o=run(&["demo","--basline","D"]);
    assert_eq!(o.status.code(),Some(2));
}
#[test]
fn req_cli_005_high_bits_are_not_silently_truncated() {
    let o=run(&["demo","--baseline","D","--word","0x10765","--evidence","test"]);
    assert_eq!(o.status.code(),Some(4));
}
#[test]
fn req_cli_006_unknown_message_does_not_fall_back_to_h() {
    let o=run(&["demo","--baseline","D","--message","SYNTH-H-ONLY","--word","42","--evidence","test"]);
    assert_eq!(o.status.code(),Some(3));
}
#[test]
fn req_cli_007_coverage_does_not_claim_real_messages() {
    let o=run(&["baselines"]);
    assert!(o.status.success());
    let text=String::from_utf8_lossy(&o.stdout);
    assert_eq!(text.lines().filter(|l|l.contains("NOT_POPULATED | 0")).count(),6);
}
