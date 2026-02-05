//! Canonical resolve_action — PoM budget enforcement.
//! Impossibility by structure, not by decision.

use super::topology::{Action, EP, IZ, RZ};
use super::types::{Budget, Effect, Impossibility, Invariants};

/// Resolve action through RZ→EP→IZ topology.
///
/// Returns `Effect` if action is instantiable within budget,
/// or `Impossibility` if structural constraints prevent instantiation.
///
/// # Invariants
/// - Capacity must cover magnitude (thermodynamic accounting)
/// - Progression must be non-zero (finite state transitions)
/// - Topology progression is enforced by typestate
#[allow(unused_variables)] // inv reserved for Phase 7 expansion
pub fn resolve_action(
    action: Action<RZ>,
    budget: &mut Budget,
    inv: Invariants,
) -> Result<Effect, Impossibility> {
    // ═══════════════════════════════════════════════════════════
    // RZ: Pre-engagement verification (reversible)
    // ═══════════════════════════════════════════════════════════
    if budget.capacity.0 < action.magnitude().0 {
        return Err(Impossibility::CapacityInsufficient);
    }

    // ═══════════════════════════════════════════════════════════
    // EP: First irreversibility — action engaged
    // ═══════════════════════════════════════════════════════════
    let engaged: Action<EP> = action.engage();

    // CANON: progression_rule(inv.flow, inv.entropy) — Phase 7 expansion
    if budget.progression.0 == 0 {
        return Err(Impossibility::ProgressionExhausted);
    }
    budget.progression.0 -= 1;

    // Capture magnitude BEFORE move into IZ
    let magnitude_applied = engaged.magnitude();

    // ═══════════════════════════════════════════════════════════
    // IZ: Effect produced — terminal state
    // ═══════════════════════════════════════════════════════════
    let _iz: Action<IZ> = engaged.into_iz();

    Ok(Effect { magnitude_applied })
}
