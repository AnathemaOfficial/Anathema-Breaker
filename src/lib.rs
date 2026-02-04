#![no_std]

// Phase 5.1: structure-only skeleton.
// Zero behavior. Zero runtime surfaces. Lexicon-only public API.

pub mod canon;
pub mod types;
pub mod pom;
pub(crate) mod api;

// ✅ PUBLIC SURFACE = CANON LEXICON ONLY
pub use types::{Progression, Capacity, Action, IZ};

// ============================================================
// Phase 5.2: smoke tests (unit tests — inside crate boundary)
// ============================================================
#[cfg(test)]
extern crate std;

#[cfg(test)]
mod tests {
    use super::{Capacity, Action, IZ, Progression};

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
        assert_eq!(core::mem::size_of::<Action>(), 32);
        assert_eq!(core::mem::size_of::<IZ>(), 32);
    }

    #[test]
    fn lexicon_compiles_inside_crate() {
        let _ = Progression::RZ;
        let _ = Capacity::new(0);
        let _ = Action::new([0u8; 32]);
        let _ = IZ::new([0u8; 32]);
    }
}
