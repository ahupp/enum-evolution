#[test]
fn removed_variant_is_gone() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/removed_variant_fail.rs");
}
