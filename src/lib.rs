#![no_std]

// Current active crate state:
// Phase 7.0 / AB-S sealed software machine built from the Phase 6.2 PoM core.
// Historical phase notes are kept in README files, not here.

pub mod canon;
pub mod types;
pub mod pom;
// pub(crate) mod api; // sealed phase 6.2: api not in build graph

// Public surface remains narrow and law-shaped.
pub use types::{Progression, Capacity, ActionEnvelope, IZ};

// ============================================================
// Legacy lexicon smoke tests (inside crate boundary)
// ============================================================
#[cfg(test)]
extern crate std;

#[cfg(test)]
mod tests {
    use super::{Capacity, ActionEnvelope, IZ, Progression};

    #[test]
    fn progression_is_byte_sized() {
        assert_eq!(core::mem::size_of::<Progression>(), 1);
    }

    #[test]
    fn capacity_is_opaque_u64() {
        assert_eq!(core::mem::size_of::<Capacity>(), 8);
    }

    #[test]
    fn envelopes_are_32_bytes() {
        assert_eq!(core::mem::size_of::<ActionEnvelope>(), 32);
        assert_eq!(core::mem::size_of::<IZ>(), 32);
    }

    #[test]
    fn lexicon_compiles_inside_crate() {
        let _ = Progression::RZ;
        let _ = Capacity::new(0);
        let _ = ActionEnvelope::new([0u8; 32]);
        let _ = IZ::new([0u8; 32]);
    }
}
