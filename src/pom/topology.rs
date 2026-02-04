//! Engagement topology — typestate enforcement of RZ→EP→IZ.
//! Invalid transitions are structurally non-representable.

use super::types::{Domain, Magnitude};

/// Reception Zone — pre-engagement, reversible.
pub struct RZ;

/// Engagement Point — first irreversibility.
pub struct EP;

/// Impossibility Zone — effect produced, terminal.
pub struct IZ;

/// Sealed marker — default topology state.
pub struct SEALED;

/// Action envelope with typestate progression.
/// State `S` determines available transitions.
/// Fields are private — no external mutation allowed.
pub struct Action<S, T = SEALED> {
    domain: Domain,
    magnitude: Magnitude,
    _state: core::marker::PhantomData<(S, T)>,
}

// ═══════════════════════════════════════════════════════════════
// Getters — immutable access only
// ═══════════════════════════════════════════════════════════════
impl<S, T> Action<S, T> {
    /// Domain identifier (read-only).
    #[inline]
    pub fn domain(&self) -> Domain {
        self.domain
    }

    /// Magnitude value (read-only).
    #[inline]
    pub fn magnitude(&self) -> Magnitude {
        self.magnitude
    }
}

// ═══════════════════════════════════════════════════════════════
// RZ: Reception Zone — entry point
// ═══════════════════════════════════════════════════════════════
impl Action<RZ> {
    /// Create action in Reception Zone.
    pub fn new(domain: Domain, magnitude: Magnitude) -> Self {
        Self {
            domain,
            magnitude,
            _state: core::marker::PhantomData,
        }
    }

    /// RZ → EP: engage action (first irreversibility).
    pub fn engage(self) -> Action<EP> {
        Action {
            domain: self.domain,
            magnitude: self.magnitude,
            _state: core::marker::PhantomData,
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// EP: Engagement Point — irreversible
// ═══════════════════════════════════════════════════════════════
impl Action<EP> {
    /// EP → IZ: produce effect (terminal).
    pub fn into_iz(self) -> Action<IZ> {
        Action {
            domain: self.domain,
            magnitude: self.magnitude,
            _state: core::marker::PhantomData,
        }
    }
}

// No impl for Action<IZ> — terminal state, no further transitions.
// No impl for Action<EP>::back_to_rz() — path absence enforced.
// No impl for Action<RZ>::into_iz() — path absence enforced.
