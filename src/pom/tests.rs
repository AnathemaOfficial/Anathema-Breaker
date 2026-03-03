//! Phase 6.2 tests — negative cases + determinism property.
//! No positive "happy path" tests — impossibility is the specification.

use super::resolve_action::resolve_action;
use super::topology::{Action, RZ};
use super::types::{Budget, Capacity, Domain, Impossibility, Magnitude, Progression};

/// Capacity insufficient → CapacityInsufficient impossibility.
#[test]
fn capacity_insufficient_is_impossibility() {
    let action = Action::<RZ>::new(Domain::new(1), Magnitude::new(10));
    let mut budget = Budget::new(Capacity::new(9), Progression::new(1));

    assert_eq!(
        resolve_action(action, &mut budget),
        Err(Impossibility::CapacityInsufficient)
    );
}

/// Progression exhausted → ProgressionExhausted impossibility.
#[test]
fn progression_exhausted_is_impossibility() {
    let action = Action::<RZ>::new(Domain::new(1), Magnitude::new(10));
    let mut budget = Budget::new(Capacity::new(10), Progression::new(0));

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
    let action1 = Action::<RZ>::new(Domain::new(7), Magnitude::new(5));
    let mut budget1 = Budget::new(Capacity::new(5), Progression::new(1));
    let out1 = resolve_action(action1, &mut budget1);

    // Second resolution — identical inputs
    let action2 = Action::<RZ>::new(Domain::new(7), Magnitude::new(5));
    let mut budget2 = Budget::new(Capacity::new(5), Progression::new(1));
    let out2 = resolve_action(action2, &mut budget2);

    // Structural determinism: same input → same output (full equality)
    assert_eq!(out1, out2);
}

/// Budget mutation occurs only on successful engagement.
/// Verify Err variant + both capacity and progression unchanged.
#[test]
fn budget_unchanged_on_capacity_impossibility() {
    let action = Action::<RZ>::new(Domain::new(1), Magnitude::new(100));
    let mut budget = Budget::new(Capacity::new(10), Progression::new(5));

    let result = resolve_action(action, &mut budget);

    // Verify correct impossibility returned
    assert_eq!(result, Err(Impossibility::CapacityInsufficient));

    // Budget unchanged — rejection in RZ, before engagement
    assert_eq!(budget.capacity().value(), 10);
    assert_eq!(budget.progression().value(), 5);
}

/// Budget must remain unchanged on ProgressionExhausted.
#[test]
fn budget_unchanged_on_progression_exhausted() {
    let action = Action::<RZ>::new(Domain::new(1), Magnitude::new(10));
    let mut budget = Budget::new(Capacity::new(10), Progression::new(0));

    let before_capacity = budget.capacity().value();
    let before_progression = budget.progression().value();

    assert_eq!(
        resolve_action(action, &mut budget),
        Err(Impossibility::ProgressionExhausted)
    );

    assert_eq!(budget.capacity().value(), before_capacity);
    assert_eq!(budget.progression().value(), before_progression);
}
