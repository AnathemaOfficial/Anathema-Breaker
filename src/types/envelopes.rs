/// Action envelope — 32-byte binary frame (Phase 4.1).
/// Renamed from `Action` to avoid confusion with `pom::topology::Action<S>`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionEnvelope([u8; 32]);

impl ActionEnvelope {
    /// Crate-internal constructor only.
    pub(crate) const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

/// IZ envelope (Phase 4.2).
/// Distinct from Progression::IZ.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IZ([u8; 32]);

impl IZ {
    /// Crate-internal constructor only.
    pub(crate) const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

/// Deployment topology identifier (internal).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct TopologyID([u8; 32]);

impl TopologyID {
    pub(crate) const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}
