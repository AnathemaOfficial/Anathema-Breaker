# MN-001-P7-MACHINE-SEALING — AB-S (Software Tier)

**Phase:** 7.0  
**Date:** 2026-02-05  
**Status:** SEALED  
**Tier:** AB-S (Software — Representational Impossibility)

---

## 0) Scope

Phase 7.0 seals Anathema-Breaker as a **formal software machine (AB-S)**.

AB-S provides **representational impossibility**: paths that violate its invariants
cannot be expressed in safe Rust within its formal boundary.

This spec is **additive to Phase 6.2**. No modification to Phase 6.2 code or
invariants is required or permitted.

---

## 1) Definition — AB-S (Software Tier)

AB-S is a sealed logical machine defined by:

- **Typestate topology:** `RZ → EP → IZ` (Phase 6.2)
- **Thermodynamic accounting:** `Capacity` / `Progression` (Phase 6.2)
- **Path absence:** Invalid transitions unrepresentable in safe Rust
- **No runtime dependencies:** `no_std` compatible

### What AB-S Guarantees

✅ **Representational impossibility**  
Paths violating invariants cannot be expressed in the type system.

✅ **Compile-stage enforcement**  
Invalid topologies fail at compile-stage (trybuild tests).

✅ **Deterministic resolution**  
Same `Budget` + same `Action` → same `Effect` (no adaptation).

### What AB-S Does NOT Guarantee

❌ **Physical impossibility**  
AB-S operates within a host environment (OS, hardware) outside its formal boundary.

❌ **Protection against host compromise**  
Memory inspection, process injection, or OS-level attacks are out of scope.

❌ **Runtime adaptation**  
AB-S has no feedback mechanism — effects cannot influence future resolutions.

---

## 2) Formal Boundary of AB-S

The boundary of AB-S is the **typestate progression** itself:

| State | Description | Transition |
|-------|-------------|------------|
| **RZ** | Reception Zone (pre-engagement, reversible) | → EP only |
| **EP** | Engagement Point (first irreversibility) | → IZ only |
| **IZ** | Terminal state (possibility ceases) | → Effect (one-way) |

### Structural Impossibilities (Enforced by Type System)

- ❌ `RZ → IZ` direct (no `Action<RZ>::into_iz()` method)
- ❌ `EP → RZ` backward (no `Action<EP>::back_to_rz()` method)
- ❌ `Effect → Action` feedback (no conversion API exists)

These are **structural absences**, not runtime checks.

---

## 3) Excitation Model

AB-S receives excitation as a **Budget** provided ex ante:

```rust
struct Budget {
    capacity: Capacity,
    progression: Progression,
}
```

### Properties

- **Opaque values:** `Capacity(u32)`, `Progression(u32)` — no semantic interpretation
- **Deterministic consumption:** Budget decremented according to fixed rules
- **No adaptation:** AB-S does not "learn" or adjust based on past resolutions

The excitation model is **non-interpretive**: AB-S consumes budget structurally,
without understanding what actions "mean".

---

## 4) Absence of Feedback

AB-S produces an `Effect` but provides **no channel** for that effect to
influence future resolutions:

```rust
struct Effect {
    magnitude_applied: Magnitude,
}
```

### Structural Guarantees

- `Effect` has no methods exposing internal state (beyond `magnitude_applied`)
- `Effect` cannot be converted back to `Action` or `Budget`
- No API exists to "report back" execution results to AB-S

Feedback absence is **structural**, not behavioral: the type system makes
feedback paths unrepresentable.

---

## 5) Invariants (Non-Negotiable)

### P7-I1 — Representational Impossibility

Invalid paths are unrepresentable in safe Rust.

**Proof:** Compile-fail tests (trybuild) demonstrate that attempts to:
- Transition `RZ → IZ` directly
- Transition `EP → RZ` backward
- Convert `Effect → Action`

...all fail at compile-stage with "no such method" errors.

### P7-I2 — No Feedback Channels

No type conversion or API permits `Effect → Action` flow.

**Proof:** `Effect` struct has no methods beyond field access. No `From<Effect>`
or `Into<Action>` trait implementations exist.

### P7-I3 — Excitation Opacity

Budget values are consumed but never interpreted semantically.

**Proof:** `resolve_action` performs only arithmetic operations on budget fields.
No conditional logic based on "meaning" of capacity/progression values.

### P7-I4 — Additive Only

Phase 7.0 extends Phase 6.2 via specification only.

**Proof:** No code modification to Phase 6.2 permitted. All Phase 7.0 additions
are in new files (`specs/`, `tests/trybuild/`, `examples/`, `docs/`).

---

## 6) Tier Declaration — Honesty About Guarantees

AB-S provides **representational impossibility** within its formal boundary.

### AB-S Tier Statement

> AB-S guarantees that unsafe paths cannot be represented in safe Rust.  
> It does NOT provide physical impossibility or protection against host environment
> compromise. For physical impossibility, see AB-R (requires dedicated hardware).

Systems integrating AB-S must respect its formal boundary: treating AB-S as a
black box whose internal structure is immutable.

### Comparison to Future Tiers

| Tier | Impossibility Type | Hardware Requirement |
|------|-------------------|---------------------|
| **AB-S** (current) | Representational (type system) | Standard computer |
| **AB-H** (future) | Structural (reduced attack surface) | Embedded/FPGA |
| **AB-R** (research) | Thermodynamic (physical constraints) | La ROCHE device |

---

## 7) Sealing Criterion

Phase 7.0 is sealed when:

1. ✅ This spec is immutable (hash/tag recorded)
2. ✅ Tests P7-S1 pass (compile-fail proof: `Effect → Action` impossible)
3. ✅ Example `p7_sealed_machine.rs` compiles and demonstrates AB-S as
   self-contained logical machine
4. ✅ No modification to Phase 6.2 code has occurred

---

## 8) Canonical Tag

**Tag:** `mn-001-p7.0-machine-sealing-20260205`  
**Spec Hash:** (compute after freeze)

---

## 9) Post-Seal Immutability

After Phase 7.0 seal:

- ✅ **Adjustable:** Documentation, examples, tier explanations (non-canonical)
- ❌ **Immutable:** This spec, Phase 6.2 code, typestate topology, impossibility claims

Any modification to immutable elements = fork, not Phase 7.0.

---

**Phase 7.0 — SEALED**

> What cannot be represented needs no control.  
> — AB-S Core Principle
