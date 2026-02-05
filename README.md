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

Phase 6.2 — Integration proof (branchable law)

Phase 6.2 demonstrates that the Anathema-Breaker law can be integrated into an external system without modification.

A minimal example shows how an external system can:

construct an Action<RZ> and Budget,

call the canonical resolve_action,

execute business logic only if an Effect is produced,

remain silent on impossibility (fail-closed, no feedback).

No new abstractions are added to the PoM core.
No semantic interpretation is introduced.
The law remains unchanged.

Phase 6.2 exists solely as a proof of branchability, not as a product interface.

See: examples/minimal_integration.rs
Tag: mn-001-p6.2-integration-20260205

See: `README_PHASE5.md`.


