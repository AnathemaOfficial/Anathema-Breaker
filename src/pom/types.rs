//! Canonical PoM types — thermodynamic accounting primitives.
//! All budget fields are private — only resolve_action may consume.

/// Domain identifier — action classification.
/// Field is private; construct via `Domain::new()`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Domain(u64);

impl Domain {
    /// Create a domain identifier.
    #[inline]
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    /// Read the raw domain value.
    #[inline]
    pub const fn value(self) -> u64 {
        self.0
    }
}

/// Magnitude — energetic cost of action.
/// Field is private; construct via `Magnitude::new()`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Magnitude(u64);

impl Magnitude {
    /// Create a magnitude value.
    #[inline]
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    /// Read the raw magnitude value.
    #[inline]
    pub const fn value(self) -> u64 {
        self.0
    }
}

/// Capacity — available thermodynamic budget.
/// Field is private — no external mutation allowed.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Capacity(u64);

impl Capacity {
    /// Create a capacity value.
    #[inline]
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    /// Read the raw capacity value.
    #[inline]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Consume capacity. Returns false if insufficient.
    /// Only callable from within the crate (resolve_action).
    #[inline]
    pub(crate) fn consume(&mut self, amount: u64) -> bool {
        if self.0 < amount {
            false
        } else {
            self.0 -= amount;
            true
        }
    }
}

/// Progression — remaining state transitions.
/// Field is private — no external mutation allowed.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Progression(u64);

impl Progression {
    /// Create a progression counter.
    #[inline]
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    /// Read the raw progression value.
    #[inline]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Check if progression is exhausted.
    #[inline]
    pub const fn is_exhausted(self) -> bool {
        self.0 == 0
    }

    /// Tick one progression step. Returns false if already exhausted.
    /// Only callable from within the crate (resolve_action).
    #[inline]
    pub(crate) fn tick(&mut self) -> bool {
        if self.0 == 0 {
            false
        } else {
            self.0 -= 1;
            true
        }
    }
}

/// Invariants — R = (F × E) / K components.
#[derive(Clone, Copy, Debug)]
pub struct Invariants {
    pub r: u32,
    pub flow: u32,
    pub entropy: u32,
}

/// Budget — combined capacity and progression limits.
/// Fields are private — only resolve_action may consume.
/// Construct via `Budget::new()`, read via getters.
#[derive(Clone, Copy, Debug)]
pub struct Budget {
    capacity: Capacity,
    progression: Progression,
}

impl Budget {
    /// Create a budget with the given capacity and progression.
    #[inline]
    pub const fn new(capacity: Capacity, progression: Progression) -> Self {
        Self { capacity, progression }
    }

    /// Read current capacity (immutable).
    #[inline]
    pub const fn capacity(&self) -> Capacity {
        self.capacity
    }

    /// Read current progression (immutable).
    #[inline]
    pub const fn progression(&self) -> Progression {
        self.progression
    }

    /// Consume capacity. Only callable from within the crate.
    #[inline]
    pub(crate) fn consume_capacity(&mut self, amount: u64) -> bool {
        self.capacity.consume(amount)
    }

    /// Tick progression. Only callable from within the crate.
    #[inline]
    pub(crate) fn tick_progression(&mut self) -> bool {
        self.progression.tick()
    }
}

/// Impossibility — structural non-instantiability (not error).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Impossibility {
    CapacityInsufficient,
    ProgressionExhausted,
    // InvalidTopology removed: unreachable in safe Rust.
    // Typestate (RZ→EP→IZ) makes invalid topology transitions
    // non-representable at compile time. See tests/ui/ for proofs.
}

/// Effect — observable outcome of resolved action.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Effect {
    pub magnitude_applied: Magnitude,
}
