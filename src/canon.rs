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
// • Budget fields are private — no external mutation
//
// Breaking these invariants = not AB Phase 6.2
// ════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Capacity(u64);

impl Capacity {
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Progression(u64);

impl Progression {
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Budget {
    capacity: Capacity,
    progression: Progression,
}

impl Budget {
    pub const fn new(capacity: u64, progression: u64) -> Self {
        Self {
            capacity: Capacity(capacity),
            progression: Progression(progression),
        }
    }

    pub const fn capacity(&self) -> Capacity {
        self.capacity
    }

    pub const fn progression(&self) -> Progression {
        self.progression
    }
}
