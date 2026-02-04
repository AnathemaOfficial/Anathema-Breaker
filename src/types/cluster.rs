use crate::types::topology::{ShieldTopology, GatePlacement};
use crate::types::capacity::Capacity;
use crate::types::envelopes::TopologyID;

/// Finite SLIME cluster — immutable after deployment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct FiniteCluster {
    volume: ClusterVolume,
    surface: ActuationSurface,
    capacity: Capacity,
    topology: ShieldTopology,
    topology_id: TopologyID,
    gates: GatePlacement,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ClusterVolume {
    _priv: (),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ActuationSurface {
    _priv: (),
}

impl FiniteCluster {
    /// Deployment-only constructor.
    pub(crate) const fn deploy(
        volume: ClusterVolume,
        surface: ActuationSurface,
        capacity: Capacity,
        topology: ShieldTopology,
        topology_id: TopologyID,
        gates: GatePlacement,
    ) -> Self {
        Self {
            volume,
            surface,
            capacity,
            topology,
            topology_id,
            gates,
        }
    }
}
