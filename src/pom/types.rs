//! Canonical PoM types — thermodynamic accounting primitives.

/// Domain identifier — action classification.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Domain(pub u16);

/// Magnitude — energetic cost of action.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Magnitude(pub u32);

/// Capacity — available thermodynamic budget.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Capacity(pub u32);

/// Progression — remaining state transitions.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Progression(pub u32);

/// Invariants — R = (F × E) / K components.
#[derive(Clone, Copy, Debug)]
pub struct Invariants {
    pub r: u32,
    pub flow: u32,
    pub entropy: u32,
}

/// Budget — combined capacity and progression limits.
#[derive(Clone, Copy, Debug)]
pub struct Budget {
    pub capacity: Capacity,
    pub progression: Progression,
}

/// Impossibility — structural non-instantiability (not error).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Impossibility {
    CapacityInsufficient,
    ProgressionExhausted,
    InvalidTopology,
}

/// Effect — observable outcome of resolved action.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Effect {
    pub magnitude_applied: Magnitude,
}
