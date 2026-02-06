//! AB-S as a self-contained logical machine
//!
//! This example demonstrates AB-S operating as a sealed machine:
//! - Fixed excitation profile (Budget provided ex ante)
//! - Deterministic resolution (no adaptation)
//! - One-way effect projection (no feedback)
//! - No external dependencies or state

use anathema_breaker_core::pom::topology::{Action, RZ};
use anathema_breaker_core::pom::types::{Budget, Capacity, Domain, Magnitude, Progression};
use anathema_breaker_core::pom::resolve_action::resolve_action;

fn main() {
    println!("=== AB-S Sealed Machine Demo ===\n");

    // Machine initialization: fixed excitation profile
    let mut machine_budget = Budget {
        capacity: Capacity(1000),
        progression: Progression(10),
    };

    println!("Initial Budget:");
    println!("  Capacity:    {}", machine_budget.capacity.0);
    println!("  Progression: {}\n", machine_budget.progression.0);

    // Machine operation: deterministic resolution
    let action1 = Action::<RZ>::new(Domain(1), Magnitude(50));
    println!("Action 1: Domain({}) Magnitude({})", action1.domain().0, action1.magnitude().0);

    match resolve_action(action1, &mut machine_budget) {
        Ok(effect) => {
            println!("  ✓ Effect produced: magnitude_applied = {}", effect.magnitude_applied.0);
            // Effect projection: one-way output (no feedback possible)
            project_effect(effect);
        }
        Err(impossibility) => {
            println!("  ✗ Impossibility: {:?}", impossibility);
        }
    }

    println!("\nBudget after Action 1:");
    println!("  Capacity:    {}", machine_budget.capacity.0);
    println!("  Progression: {}\n", machine_budget.progression.0);

    // Second action with same parameters
    let action2 = Action::<RZ>::new(Domain(1), Magnitude(50));
    println!("Action 2: Domain({}) Magnitude({}) [identical to Action 1]", 
             action2.domain().0, action2.magnitude().0);

    match resolve_action(action2, &mut machine_budget) {
        Ok(effect) => {
            println!("  ✓ Effect produced: magnitude_applied = {}", effect.magnitude_applied.0);
            project_effect(effect);
        }
        Err(impossibility) => {
            println!("  ✗ Impossibility: {:?}", impossibility);
        }
    }

    println!("\nBudget after Action 2:");
    println!("  Capacity:    {}", machine_budget.capacity.0);
    println!("  Progression: {}\n", machine_budget.progression.0);

    // Third action exceeding capacity
    let action3 = Action::<RZ>::new(Domain(1), Magnitude(2000));
    println!("Action 3: Domain({}) Magnitude({}) [exceeds capacity]", 
             action3.domain().0, action3.magnitude().0);

    match resolve_action(action3, &mut machine_budget) {
        Ok(effect) => {
            println!("  ✓ Effect produced: magnitude_applied = {}", effect.magnitude_applied.0);
            project_effect(effect);
        }
        Err(impossibility) => {
            println!("  ✗ Impossibility: {:?} [structural impossibility, not runtime error]", impossibility);
        }
    }

    println!("\n=== Key Properties Demonstrated ===");
    println!("✓ Deterministic: same Action + same Budget → same Effect");
    println!("✓ No feedback: Effects cannot influence future resolutions");
    println!("✓ Structural impossibility: capacity exceeded → representationally impossible");
    println!("✓ Immutable law: Budget accounting rules are fixed");
}

/// Project effect to external world (one-way boundary)
///
/// In AB-S, this is a formal boundary — no implementation required.
/// In AB-R (hardware tier), this would be physical actuation.
fn project_effect(effect: anathema_breaker_core::pom::types::Effect) {
    // Physical projection would occur here in AB-R
    // In AB-S, we simply acknowledge the effect crossed the boundary
    let _ = effect.magnitude_applied;
    // Note: No way to send information back to AB-S from here
}
