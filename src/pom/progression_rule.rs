//! CANON: progression_rule(flow, entropy)
//! Source of truth: specs/phase-6-invariants.md (sealed MN-001-SYFCORE-...).
//!
//! ⚠️ STUB — Phase 6.1 placeholder only.
//! Actual rule must be extracted verbatim from sealed specs/ before Phase 7.
//! No arbitrary implementation permitted.

use super::types::{Invariants, Progression};

/// Compute progression cost from invariants.
///
/// # Panics
/// Always panics in Phase 6.1 — rule not yet extracted from sealed specs.
#[inline]
pub fn progression_cost(_inv: Invariants) -> Progression {
    panic!("CANON: progression_rule not yet extracted from sealed specs/");
}
