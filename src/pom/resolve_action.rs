use crate::pom::topology::{Action, RZ};
use crate::pom::types::{Budget, Effect, Impossibility};

/// Resolve action through RZ→EP→IZ topology.
///
/// Returns `Effect` if action is instantiable within budget,
/// or `Impossibility` if structural constraints prevent instantiation.
///
/// # Invariants (Phase 6.2)
/// - Capacity must cover magnitude (thermodynamic accounting)
/// - Progression must be non-zero (finite state transitions)
/// - Topology progression is enforced by typestate
/// - Budget fields are private — only this function consumes
pub fn resolve_action(
    action: Action<RZ>,
    budget: &mut Budget,
) -> Result<Effect, Impossibility> {
    // Capacity check: insufficiency → structural impossibility
    if budget.capacity().value() < action.magnitude().value() {
        return Err(Impossibility::CapacityInsufficient);
    }

    // Progression check: exhaustion → structural impossibility
    if budget.progression().is_exhausted() {
        return Err(Impossibility::ProgressionExhausted);
    }

    // Commit budget only on success (via crate-internal API)
    budget.consume_capacity(action.magnitude().value());
    budget.tick_progression();

    // Topology enforcement: RZ → EP → IZ (typestate path absence)
    let ep = action.engage();
    let iz = ep.into_iz();
    Ok(Effect {
        magnitude_applied: iz.magnitude(),
    })
}
