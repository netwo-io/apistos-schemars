#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("schemars/tests/ui/*.rs");
}
