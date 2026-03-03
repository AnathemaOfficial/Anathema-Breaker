//! Compile-fail tests for typestate enforcement and API sealing.
//! Invalid transitions and external mutations must fail at compile-stage.

#[test]
fn typestate_invalid_transitions_compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/rz_direct_to_iz.rs");
    t.compile_fail("tests/ui/ep_back_to_rz.rs");
    t.compile_fail("tests/trybuild/p7_no_effect_to_action.rs");
}

#[test]
fn budget_sealing_compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/budget_cheat_impossible.rs");
}
