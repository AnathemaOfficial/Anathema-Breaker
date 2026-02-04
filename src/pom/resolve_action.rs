//! Canonical resolve_action — PoM budget enforcement.
//! Impossibility by structure, not by decision.

use super::progression_rule::progression_cost;
use super::types::{Budget, Effect, Impossibility, Invariants};
use super::topology::{Action, EP, IZ, RZ};

/// Resolve action through RZ→EP→IZ topology.
///
/// Returns `Effect` if action is instantiable within budget,
/// or `Impossibility` if structural constraints prevent instantiation.
///
/// # Invariants
/// - Capacity must cover magnitude (thermodynamic accounting)
/// - Progression must cover cost (finite state transitions)
/// - Topology progression is enforced by typestate
///
/// # Panics
/// Panics in Phase 6.1 if `progression_cost` stub is called during execution.
pub fn resolve_action(
    action: Action<RZ>,
    budget: &mut Budget,
    inv: Invariants,
) -> Result<Effect, Impossibility> {
    // ═══════════════════════════════════════════════════════════
    // RZ: Pre-engagement verification (reversible)
    // ═══════════════════════════════════════════════════════════
    if budget.capacity.0 < action.magnitude.0 {
        return Err(Impossibility::CapacityInsufficient);
    }

    // ═══════════════════════════════════════════════════════════
    // EP: First irreversibility — action engaged
    // ═══════════════════════════════════════════════════════════
    let engaged: Action<EP> = action.engage();

    // Capture magnitude before EP→IZ consumes the action.
    let magnitude = engaged.magnitude;

    // CANON: progression_rule(inv.flow, inv.entropy)
    // Stub placeholder — actual rule must be extracted verbatim from sealed specs/ in Phase 7.
    let cost = progression_cost(inv).0;

    if budget.progression.0 < cost {
        return Err(Impossibility::ProgressionExhausted);
    }
    budget.progression.0 -= cost;

    // ═══════════════════════════════════════════════════════════
    // IZ: Effect produced — terminal state
    // ═══════════════════════════════════════════════════════════
    let _iz: Action<IZ> = engaged.into_iz();

    Ok(Effect {
        magnitude_applied: magnitude,
    })
}
