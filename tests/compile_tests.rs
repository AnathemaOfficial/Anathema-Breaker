//! Compile-fail tests for typestate enforcement.
//! Invalid topology transitions must fail at compile-stage.

#[test]
fn typestate_invalid_transitions_compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/rz_direct_to_iz.rs");
    t.compile_fail("tests/ui/ep_back_to_rz.rs");
}
