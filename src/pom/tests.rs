//! Phase 6.2 tests — negative cases + determinism property.
//! No positive "happy path" tests — impossibility is the specification.

use super::resolve_action::resolve_action;
use super::topology::{Action, RZ};
use super::types::{Budget, Capacity, Domain, Impossibility, Magnitude, Progression};

/// Capacity insufficient → CapacityInsufficient impossibility.
#[test]
fn capacity_insufficient_is_impossibility() {
    let action = Action::<RZ>::new(Domain(1), Magnitude(10));
    let mut budget = Budget {
        capacity: Capacity(9),
        progression: Progression(1),
    };

    assert_eq!(
        resolve_action(action, &mut budget),
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

    assert_eq!(
        resolve_action(action, &mut budget),
        Err(Impossibility::ProgressionExhausted)
    );
}

/// Determinism property: identical inputs → identical outputs.
/// This is a structural requirement, not a positive test.
#[test]
fn determinism_property_preserved() {
    // First resolution
    let action1 = Action::<RZ>::new(Domain(7), Magnitude(5));
    let mut budget1 = Budget {
        capacity: Capacity(5),
        progression: Progression(1),
    };
    let out1 = resolve_action(action1, &mut budget1);

    // Second resolution — identical inputs
    let action2 = Action::<RZ>::new(Domain(7), Magnitude(5));
    let mut budget2 = Budget {
        capacity: Capacity(5),
        progression: Progression(1),
    };
    let out2 = resolve_action(action2, &mut budget2);

    // Structural determinism: same input → same output (full equality)
    assert_eq!(out1, out2);
}

/// Budget mutation occurs only on successful engagement.
/// Verify Err variant + both capacity and progression unchanged.
#[test]
fn budget_unchanged_on_capacity_impossibility() {
    let action = Action::<RZ>::new(Domain(1), Magnitude(100));
    let mut budget = Budget {
        capacity: Capacity(10),
        progression: Progression(5),
    };

    let result = resolve_action(action, &mut budget);

    // Verify correct impossibility returned
    assert_eq!(result, Err(Impossibility::CapacityInsufficient));

    // Budget unchanged — rejection in RZ, before engagement
    assert_eq!(budget.capacity.0, 10);
    assert_eq!(budget.progression.0, 5);
}
