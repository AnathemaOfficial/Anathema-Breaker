mod progression;
mod capacity;
pub(crate) mod envelopes;
mod topology;
mod cluster;

// ✅ PUBLIC LEXICON — nothing else escapes
pub use progression::Progression;
pub use capacity::Capacity;
pub use envelopes::{ActionEnvelope, IZ};
