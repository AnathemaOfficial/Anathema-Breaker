# Anathema-Breaker

Phase 5.1 (MN-001) is a **structure-only** Rust `no_std` skeleton: **types + signatures only**.
Zero behavior. Zero runtime surfaces. Not an engine.

Phase 6.0 introduces the first executable Proof-of-Math core (RZ→EP→IZ), enforcing structural impossibility at the point of effect.

### Phase 6.1 — Compile-time proof of impossibility

Phase 6.1 establishes that invalid state transitions in Anathema-Breaker
are not representable in Rust.

This is proven via typestate and compile-fail tests.
No runtime checks are used.
No behavior is introduced.

Phase 6.1 exists solely as a proof.
It is not an integration surface.


See: `README_PHASE5.md`.


