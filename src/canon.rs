// ════════════════════════════════════════════════════════════════════
// AB PHASE 6.2 — SEALED 2026-02-06
// ════════════════════════════════════════════════════════════════════
//
// LAW LAYER (immutable)
// • Topology v0.2: RZ → EP → IZ enforced by typestate
// • Path absence: no representation of invalid transitions
// • Thermodynamic accounting: Capacity + Progression
// • Insufficiency ≠ Exhaustion (PoM semantics)
//
// MECHANISM LAYER (minimal)
// • resolve_action: deterministic, stateless per call
// • no_std compliant
// • Zero behavior beyond structural constraints
//
// INVARIANTS (non-negotiable)
// • No extension hooks
// • No configuration surface
// • No feedback channels
//
// Breaking these invariants = not AB Phase 6.2
// ════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Capacity(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Progression(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Budget {
    pub capacity: Capacity,
    pub progression: Progression,
}

impl Budget {
    pub const fn new(capacity: u64, progression: u64) -> Self {
        Self {
            capacity: Capacity(capacity),
            progression: Progression(progression),
        }
    }
}