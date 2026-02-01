# SPEC_PHASE4_1_ACTION_OPAQUE.md
## Anathema-Breaker — Action Opaque Envelope (PoM-Compliant)
*Phase 4.1 — Conceptual Specification Only*  
*No code. Format + Constraints + Structural Proof.*

---

## 0. Canon Anchors (Non-Negotiable)

| Anchor | Meaning |
|--------|---------|
| **No Semantics** | `Action` carries **no intent**, no goals, no policy, no natural language |
| **Temporal Ban** | No `time`, `TTL`, `expiry`, `timeout`, counters-as-time |
| **Fixed Envelope** | `Action` format is **immutable by specification**, not configurable |
| **Path Absence** | `Action` cannot introduce new actuation paths or meta-channels |
| **Fail-Closed** | Invalid `Action` resolves to `None` without progression or corruption |
| **PoM Inspectability** | Envelope format and validation rules are fully inspectable |

Violation of any anchor = non-Breaker system.

---

## 1. Definition

`Action` is an **opaque actuation request token**.

It is not a command language.  
It is not a prompt.  
It is not a tool-call carrier.  
It is not a policy container.

`Action` exists only to provide a **deterministic, bounded identifier** that may be evaluated for admissibility **without interpreting meaning**.

---

## 2. Canonical Envelope Format (Immutable)

### 2.1 Fixed-Size Byte Array

`Action` is defined **canonically** as a 32-byte opaque array:

```rust
struct Action([u8; 32]);
```

**Rationale for fixed size**:

* Small enough for `no_std` auditability
* Large enough for stable identifier space
* Eliminates unbounded payload risks
* Prevents hidden channels via variable-length encoding

**Critical**: The size `32` is **fixed by specification**, not recommended.  
No implementation may alter this size without becoming a non-Breaker system.

### 2.2 No Alternatives Permitted

Variable-length encodings, compression, nested structures, or external references are **forbidden**.

The envelope format is **immutable** — no negotiation, no versioning, no runtime adaptation.

---

## 3. Core Properties (Structural Only)

An admissible `Action` MUST be:

* **Opaque** — no exposed fields that invite semantic interpretation
* **Bounded** — exactly 32 bytes, no more, no less
* **Deterministic** — same bytes → same validation result, always
* **Non-temporal** — no time-like primitives in any byte position
* **Non-extensible** — no mechanism to add fields or semantics at runtime

---

## 4. Validation Rules (Total & Deterministic)

### 4.1 Structural Validity

An `Action` is **valid** if and only if:

1. Length equals 32 bytes (structural invariant)
2. Not a reserved sentinel value (see §5)
3. Contains no temporal primitives (see §6)

Validation is **total** — never panics, never throws.

### 4.2 Fail-Closed Semantics

* Valid `Action` → may proceed to `resolve_action` evaluation
* Invalid `Action` → immediately resolves to `(current_progression, current_capacity, None)`
  * No capacity consumption
  * No progression advancement
  * No state corruption

---

## 5. Reserved Sentinel Values (Forbidden)

The following byte patterns are **structurally forbidden**:

| Pattern | Bytes | Reason |
|---------|-------|--------|
| NULL | `[0x00; 32]` | Prevents implicit "no-op" escalation |
| ALL-ONES | `[0xFF; 32]` | Prevents sentinel abuse |
| MAGIC_PREFIX | First 4 bytes = `0xDEADBEEF` | Prevents hidden override channels |

Presence of any forbidden pattern → immediate `None` resolution.

---

## 6. Temporal Ban (Concrete Enforcement)

`Action` MUST NOT encode:

* Timestamps (Unix, ISO, or custom)
* Expiration counters
* Sequence numbers interpreted as freshness
* Nonces requiring external clock validation

**Enforcement**: Any validation rule that references system time or external state is **non-canon**.

---

## 7. Relationship to `resolve_action`

`Action` is a pure input token to:

```rust
resolve_action(
    p: Progression,
    c: Capacity,
    a: Action
) -> (Progression, Capacity, Option<IZ>);
```

Constraints on interpretation:

* `resolve_action` MUST NOT decode `Action` into semantics
* `resolve_action` MAY treat `Action` as:
  * a deterministic selector for a predeclared shield circuit
  * a membership test in a fixed admissibility set
* `resolve_action` MUST NOT treat `Action` as:
  * an instruction language
  * a tool-call specification
  * a policy expression

---

## 8. Proof Sketch — Path Absence Preservation

### Claim
> `Action` cannot introduce new actuation paths.

### Structural Argument

1. `Action` is a fixed 32-byte array with deterministic validation
2. Validation rules are structural only (length, sentinels) — no executable content
3. No field encodes time, override, or external reference
4. Therefore, `Action` can only select among **pre-existing, deployment-declared** topological routes inside Shield/Gate

**Conclusion**:  
`Action` does not extend the system topology — it only points at what already exists by law.  
No new path can be expressed or created.

---

## Appendix: Forbidden Surfaces (Non-Goals)

The following constitute **non-Breaker implementations**:

- ❌ Variable-length `Action` payloads
- ❌ JSON/YAML/nested structures inside `Action`
- ❌ Runtime negotiation of envelope format
- ❌ Semantic interpretation of byte patterns
- ❌ Time-based validation rules
- ❌ "Admin tokens" or privileged prefixes
- ❌ Any mechanism to alter the 32-byte fixed size

Presence of any forbidden surface = non-Breaker system.

---

*Document canonique — Phase 4.1 Action Opaque*  
*MN-001-SYFCORE-20260201-AB-P4.1-SEALED*
