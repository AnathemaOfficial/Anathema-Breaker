# SPEC_PHASE4_2_IZ_ENVELOPE.md
## Anathema-Breaker — IZ Effect Envelope (PoM-Compliant)
*Phase 4.2 — Conceptual Specification Only*  
*No code. Format + Constraints + Structural Proof.*

---

## 0. Canon Anchors (Non-Negotiable)

| Anchor | Meaning |
|--------|---------|
| **Topological IZ** | `IZ` is first a terminal domain in `RZ → EP → IZ` topology (SEALED Phase 4.4) |
| **No Semantics** | Envelope carries no intent, goals, policy, or natural-language meaning |
| **Temporal Ban** | No `time`, `TTL`, `expiry`, `timeout`, counters-as-time |
| **Fixed Envelope** | Format is immutable by specification, not configurable |
| **Thermodynamic Dissipation** | Effects emitted from domain `IZ` are dissipated — cannot re-enter the cluster |
| **No Feedback Path** | Envelope must not encode or enable a return channel into `Progression`/`Capacity` |
| **Thermodynamic Accounting** | Emission implies internal capacity consumption (δ > 0) and monotonic progression |
| **PoM Inspectability** | Format and validation rules are fully inspectable |

Violation of any anchor = non-Breaker system.

---

## 1. Definition

The **IZ envelope** (`struct IZ([u8; 32])`) is the observable representation of an effect that has traversed the terminal domain `IZ`.

Critical distinction:

* **`IZ` (topological)** = terminal domain in `RZ → EP → IZ` progression (irreversible zone)
* **`IZ` (envelope)** = 32-byte opaque representation of an effect emitted from that domain

The envelope is not:

* a result value or explanation
* a log stream or telemetry
* a command language or control signal
* a feedback message or capability token

It is a **thermodynamically dissipated effect artifact** — it has left the cluster and cannot return.

---

## 2. Canonical Envelope Format (Immutable)

### 2.1 Fixed-Size Byte Array

The IZ envelope is defined canonically as a fixed 32-byte opaque array:

```rust
struct IZ([u8; 32]);
```

**Critical:** the size `32` is fixed by specification — not recommended, not configurable.  
No implementation may alter this size without becoming a non-Breaker system.

Rationale:

* Bounded, auditable handling in `no_std`
* Prevents hidden channels via variable-length encoding
* Supports decidable equality and deterministic inspection
* Aligns with `Action` envelope size for symmetry (not required, but simplifies PoM)

### 2.2 No Alternatives Permitted

The following are strictly forbidden:

* Variable-length payloads or streaming
* Compression or encoding schemes
* Nested structures (JSON/YAML/CBOR/etc.)
* External references (URLs, handles, file paths)
* Chunking or segmentation

The envelope format is immutable: no negotiation, no versioning, no runtime adaptation.

---

## 3. Structural Validity Rules (Total & Deterministic)

An `IZ` envelope is valid if and only if:

1. Length equals exactly 32 bytes (structural invariant)
2. Not a reserved sentinel pattern (see §4)
3. Contains no temporal primitives (see §5)

Validation is **total**: never panics, never throws, never depends on external state.

Invalid envelopes MUST be treated as **non-actuating output** (`None`) at the boundary — no effect emitted.

---

## 4. Reserved Sentinel Values (Forbidden)

The following byte patterns are structurally forbidden:

| Pattern | Bytes | Reason |
|---------|-------|--------|
| NULL | `[0x00; 32]` | Prevents ambiguous "silent actuation" |
| ALL-ONES | `[0xFF; 32]` | Prevents sentinel abuse |
| MAGIC_PREFIX | First 4 bytes = `0xDEADBEEF` | Prevents hidden meta-channels |

Presence of any forbidden pattern → output MUST be treated as `None`.

---

## 5. Temporal Ban (Concrete Enforcement)

The envelope MUST NOT encode:

* Timestamps (Unix, ISO, custom)
* Expiration markers
* Monotonic counters interpreted as time/freshness
* Nonces requiring external clock validation

Any implementation using system time to produce or validate the envelope is non-canon.

---

## 6. Relationship to `resolve_action` (Phase 3)

The envelope appears only as output:

```rust
resolve_action(p: Progression, c: Capacity, a: Action)
  -> (Progression, Capacity, Option<IZ>);
```

Constraints:

* `IZ` MUST NEVER appear as input to any function that updates `Progression` or `Capacity`
* `IZ` MUST NEVER be convertible to `Action` or any input type
* `IZ` MUST NEVER be interpretable as instructions, control signals, or capability tokens
* `IZ` is **output-only and dissipated** — it has left the thermodynamic boundary of the cluster

---

## 7. Thermodynamic Accounting Link (Structural)

**Emission condition:**

* `Some(IZ)` is emitted only when internal thermodynamic consumption `δ > 0` occurs
* Emission implies monotonic progression to domain `IZ` (terminal)

**Observation constraint:**

* Callers observe only `IZ([u8; 32])`
* Callers NEVER observe `δ`, `c - δ`, or any internal accounting state
* Capacity arithmetic remains fully opaque

This preserves Phase 3 requirements:

* Capacity opacity
* Progression only on consumption
* `insufficiency ≠ exhaustion`

---

## 8. Proof Sketch — No Feedback Path (Path Absence Preservation)

### Claim
> No feedback channel can exist from `IZ` envelope back into the cluster.

### Structural Argument

1. The envelope is emitted from domain `IZ` — the terminal, irreversible zone
2. By SLIME law, effects leaving domain `IZ` are **thermodynamically dissipated**
3. Dissipation is a property of the milieu, not a runtime rule:
   * The cluster boundary is a one-way thermodynamic sink
   * No physical path exists for re-entry
4. Type-level enforcement:
   * No function accepts `IZ` as input for progression/capacity updates
   * No conversion `IZ → Action` is defined or possible
   * No interpretation of envelope content is permitted

**Conclusion:**
A feedback path is not prohibited — it is **structurally undefined by the physics of the milieu**.

> *Dissipation is not a policy. It is a geometric fact of the finite cluster.*

---

## Appendix: Forbidden Surfaces (Non-Goals)

Presence of any of the following constitutes a non-Breaker implementation:

- ❌ Variable-length or streamed `IZ` output
- ❌ `IZ` interpreted as command/control channel
- ❌ Any API accepting `IZ` to modify `Progression`/`Capacity`
- ❌ Time-based validation or generation of `IZ`
- ❌ "Ack" protocols, retries, or handshakes coupled to `IZ`
- ❌ Conversion functions `IZ → Action` or `IZ → any input type`
- ❌ Any mechanism suggesting `IZ` can re-enter the cluster

Presence of any forbidden surface = non-Breaker system.

---

*Document canonique — Phase 4.2 IZ Envelope*  
*MN-001-SYFCORE-20260201-AB-P4.2-SEALED*
