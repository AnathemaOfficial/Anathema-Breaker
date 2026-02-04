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
pub struct Action<S, T = SEALED> {
    pub domain: Domain,
    pub magnitude: Magnitude,
    _state: core::marker::PhantomData<(S, T)>,
}

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
