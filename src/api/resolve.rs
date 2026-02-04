use crate::types::{Action, Capacity, IZ, Progression};

/// Phase 3 canonical signature:
/// (Progression, Capacity, Action)
///   → (Progression, Capacity, Option<IZ>)
///
/// Phase 5.1: signature only.
pub(crate) trait ResolveAction {
    fn resolve_action(
        &self,
        p: Progression,
        c: Capacity,
        a: Action,
    ) -> (Progression, Capacity, Option<IZ>);
}
