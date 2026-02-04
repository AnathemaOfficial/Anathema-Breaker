//! Canon markers — immutable structural identifiers.
//! Not configuration. Not runtime. Not negotiable.

/// Phase 5.1 skeleton seal (MN-001, 2026-02-03).
pub const AB_PHASE5_SKELETON_SEAL: &str =
    "MN-001-SYFCORE-20260203-AB-P5-SKELETON";

/// Canon derivation algorithm (Phase 4.3).
/// Fixed by law: BLAKE3-256.
pub enum CanonDerivationAlgo {
    Blake3_256,
}
