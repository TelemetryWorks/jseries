#[test]
fn public_api_usage_compiles() {
    let cases = trybuild::TestCases::new();
    cases.pass("tests/ui/public_api.rs");
}
