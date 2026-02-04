/// Opaque thermodynamic accounting unit.
/// No arithmetic. No semantics exposed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Capacity(u64);

impl Capacity {
    /// Crate-internal constructor only.
    pub(crate) const fn new(raw: u64) -> Self {
        Self(raw)
    }
}
