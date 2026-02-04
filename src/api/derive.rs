use crate::canon::CanonDerivationAlgo;
use crate::types::envelopes::TopologyID;
use crate::types::{Action, IZ};

/// Phase 4.3 canonical derivation signature.
/// Algorithm fixed by law: BLAKE3-256.
pub(crate) trait DeriveIZ {
    const ALGO: CanonDerivationAlgo = CanonDerivationAlgo::Blake3_256;

    fn derive_iz(&self, action: Action, topology_id: TopologyID) -> Option<IZ>;
}
