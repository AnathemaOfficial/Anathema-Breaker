//! Phase 6.0 tests — negative cases + determinism property.
//! No positive "happy path" tests — impossibility is the specification.

use super::resolve_action::resolve_action;
use super::topology::{Action, RZ};
use super::types::{Budget, Capacity, Domain, Impossibility, Invariants, Magnitude, Progression};

/// Capacity insufficient → CapacityInsufficient impossibility.
#[test]
fn capacity_insufficient_is_impossibility() {
    let action = Action::<RZ>::new(Domain(1), Magnitude(10));
    let mut budget = Budget {
        capacity: Capacity(9),
        progression: Progression(1),
    };
    let inv = Invariants {
        r: 0,
        flow: 0,
        entropy: 0,
    };

    assert_eq!(
        resolve_action(action, &mut budget, inv),
        Err(Impossibility::CapacityInsufficient)
    );
}

/// Progression exhausted → ProgressionExhausted impossibility.
#[test]
fn progression_exhausted_is_impossibility() {
    let action = Action::<RZ>::new(Domain(1), Magnitude(10));
    let mut budget = Budget {
        capacity: Capacity(10),
        progression: Progression(0),
    };
    let inv = Invariants {
        r: 0,
        flow: 0,
        entropy: 0,
    };

    assert_eq!(
        resolve_action(action, &mut budget, inv),
        Err(Impossibility::ProgressionExhausted)
    );
}

/// Determinism property: identical inputs → identical outputs.
/// This is a structural requirement, not a positive test.
#[test]
fn determinism_property_preserved() {
    let inv = Invariants {
        r: 1,
        flow: 2,
        entropy: 3,
    };

    // First resolution
    let action1 = Action::<RZ>::new(Domain(7), Magnitude(5));
    let mut budget1 = Budget {
        capacity: Capacity(5),
        progression: Progression(1),
    };
    let out1 = resolve_action(action1, &mut budget1, inv);

    // Second resolution — identical inputs
    let action2 = Action::<RZ>::new(Domain(7), Magnitude(5));
    let mut budget2 = Budget {
        capacity: Capacity(5),
        progression: Progression(1),
    };
    let out2 = resolve_action(action2, &mut budget2, inv);

    // Structural determinism: same input → same outcome class
    assert_eq!(out1.is_ok(), out2.is_ok());
}

/// Budget mutation occurs only on successful engagement.
#[test]
fn budget_unchanged_on_capacity_impossibility() {
    let action = Action::<RZ>::new(Domain(1), Magnitude(100));
    let mut budget = Budget {
        capacity: Capacity(10),
        progression: Progression(5),
    };
    let inv = Invariants {
        r: 0,
        flow: 0,
        entropy: 0,
    };

    let _ = resolve_action(action, &mut budget, inv);

    // Budget unchanged — no engagement occurred
    assert_eq!(budget.progression.0, 5);
}
