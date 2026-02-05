use crate::types::{Action, Budget, Effect, Impossibility, RZ};

/// Resolve action through RZ→EP→IZ topology.
///
/// Returns `Effect` if action is instantiable within budget,
/// or `Impossibility` if structural constraints prevent instantiation.
///
/// # Invariants (Phase 6.2)
/// - Capacity must cover magnitude (thermodynamic accounting)
/// - Progression must be non-zero (finite state transitions)
/// - Topology progression is enforced by typestate
pub fn resolve_action(
    action: Action<RZ>,
    budget: &mut Budget,
) -> Result<Effect, Impossibility> {
    // Capacity accounting: insufficiency ≠ exhaustion
    if budget.capacity.0 < action.magnitude().0 {
        return Err(Impossibility::CapacityInsufficient);
    }
    budget.capacity.0 -= action.magnitude().0;

    // Progression accounting: finite state transitions
    if budget.progression.0 == 0 {
        return Err(Impossibility::ProgressionExhausted);
    }
    budget.progression.0 -= 1;

    // Topology enforcement: RZ → EP → IZ (typestate path absence)
    let ep = action.engage();
    let iz = ep.into_iz();
    Ok(Effect {
        magnitude_applied: iz.magnitude(),
    })
}