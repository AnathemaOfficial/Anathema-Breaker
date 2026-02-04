//! Phase 6.1 tests — negative cases only.
//! Tests that trigger progression_cost stub are disabled until Phase 7.
//!
//! ⚠️ resolve_action panics at runtime if it reaches progression_cost.
//! Only RZ-phase impossibilities (CapacityInsufficient) can be tested.

use super::resolve_action::resolve_action;
use super::topology::{Action, RZ};
use super::types::{Budget, Capacity, Domain, Impossibility, Invariants, Magnitude, Progression};

/// Capacity insufficient → CapacityInsufficient impossibility.
/// This test is safe: rejection happens in RZ before stub is called.
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

/// Budget unchanged on RZ rejection — no engagement occurred.
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

    // Budget unchanged — rejection in RZ, before engagement
    assert_eq!(budget.progression.0, 5);
    assert_eq!(budget.capacity.0, 10);
}

// ═══════════════════════════════════════════════════════════════════
// DISABLED: Tests that would call progression_cost stub (panic)
// Re-enable in Phase 7 when actual rule is extracted from specs/
// ═══════════════════════════════════════════════════════════════════

// #[test]
// fn progression_exhausted_is_impossibility() { ... }

// #[test]
// fn determinism_property_preserved() { ... }
