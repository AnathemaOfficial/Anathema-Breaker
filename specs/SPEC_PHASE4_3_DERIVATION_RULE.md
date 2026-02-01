# SPEC_PHASE4_3_DERIVATION_RULE.md
## Anathema-Breaker — Internal Action → IZ Derivation (PoM-Compliant)
*Phase 4.3 — Conceptual Specification*  
*Strict Spec — No Code, No Alternatives*

---

## 0. Scope & Status

* **Scope**: Internal derivation rule inside Anathema-Breaker
* **Layer**: SLIME / Anathema-Breaker Core
* **Status**: Canonical Specification (Strict)
* **Non-Goals**: Optimization, extensibility, alternative algorithms

This document defines **the only admissible derivation rule** transforming an `Action` envelope into an `IZ` envelope **inside** the finite cluster.

---

## 1. Canon Anchors (Non-Negotiable)

| Anchor | Definition |
|--------|------------|
| **No Semantics** | No interpretation of `Action` content (no intent, no meaning) |
| **Temporal Ban** | No time, TTL, expiry, freshness, counters-as-time |
| **Fixed Rule** | Exactly one derivation rule — no alternatives |
| **Deterministic** | Same inputs → same output, always |
| **Topology-Bound** | Derivation limited to deployment-declared topology |
| **Thermodynamic Cost** | Successful derivation implies internal δ > 0 |
| **Dissipation** | `IZ` envelope exits cluster and cannot return |
| **No Conversion** | No `IZ → Action` conversion exists |
| **Path Absence** | No new actuation paths may be created |

Violation of any anchor = **non-Breaker system**.

---

## 2. Canonical Inputs (Already Sealed)

This phase introduces **no new external inputs**.

### 2.1 Fixed Inputs

| Input | Definition | Source |
|-------|------------|--------|
| `Action` | `[u8; 32]` opaque envelope | Phase 4.1 SEALED |
| `Progression` | `RZ \| EP \| IZ` | Phase 3 SEALED |
| `Capacity` | Opaque thermodynamic unit | Phase 3 SEALED |
| `ShieldTopology` | Deployment-declared routing | Phase 3 SEALED |

### 2.2 Derived Identifier

A **TopologyID** is derived **once at deployment**:

```
TopologyID : [u8; 32]
```

* Deterministically derived from `ShieldTopology`
* Immutable for the lifetime of the cluster
* Never exposed externally
* Not configurable

---

## 3. Derivation Rule (Canonical, Immutable)

### 3.1 Rule Signature

```rust
derive_iz(action: Action, topology_id: [u8; 32]) -> Option<IZ>;
```

This function is **internal** to the cluster and **never exposed** as a public interface.

### 3.2 Admissibility Gate

Let:

```
AdmissibleSet(topology_id) ⊆ {0,1}^{256}
```

Where:

* The set is **declared at deployment**
* The set is **finite**
* The set is **immutable**
* No runtime expansion or mutation is permitted

### 3.3 Canonical Derivation Law

Derivation is defined **exclusively** as follows:

**1. Membership Test**

```
If action ∉ AdmissibleSet(topology_id) → None
```

**2. Derivation**

```
If action ∈ AdmissibleSet(topology_id) →
    Some( IZ( H(action || topology_id) ) )
```

Where:

* `||` denotes concatenation
* `H` is the canonical compression function defined below

No other transformation is permitted.

### 3.4 Canonical Compression Function `H`

The compression function is **fixed by law**:

```
H(x) = BLAKE3-256(x)
```

Properties:

* Deterministic
* Fixed 32-byte output
* Constant-time implementable
* Auditable in `no_std`

No alternative hash, no parameterization, no versioning is allowed.

---

## 4. Thermodynamic Accounting Coupling

### 4.1 Consumption Rule

Internal thermodynamic consumption `δ > 0` occurs **iff**:

```
derive_iz(action, topology_id) = Some(IZ)
```

### 4.2 Opacity Preservation

External observers may see:

* `Action([u8;32])`
* `Option<IZ([u8;32])>`
* Updated opaque `Capacity`

They **never** see:

* δ
* Intermediate values
* Membership proof
* TopologyID

This preserves Phase 3 accounting invariants.

---

## 5. Dissipation & No-Feedback Law

### 5.1 Topological Dissipation

* `IZ` envelope is emitted **from** topological domain `IZ`
* Domain `IZ` is **terminal and irreversible**
* The envelope **leaves the finite cluster boundary**

By construction of the milieu:

> No physical or logical path exists for re-entry.

### 5.2 Structural No-Conversion Rules

The following functions **must not exist**:

* `IZ → Action`
* `IZ → Progression`
* `IZ → Capacity`
* `IZ → any cluster input`

No acknowledgement, retry, or handshake may be coupled to `IZ`.

A feedback path is not forbidden — it is **undefined**.

---

## 6. Path Absence Proof (Strict)

### Claim
> The derivation rule cannot create new actuation paths.

### Proof

1. Outputs are defined as:

```
Image = { H(action || topology_id) | action ∈ AdmissibleSet(topology_id) }
```

2. `AdmissibleSet` is fixed at deployment.
3. `H` is fixed and total.
4. Therefore `Image` is finite and fixed.
5. No input outside `AdmissibleSet` can produce an output.
6. No output can be transformed into a new input.

**Conclusion:**
The actuation surface is exactly the deployment-declared image.
No extension, bypass, or emergent path is possible.

---

## 7. Integration with Phase 3

The derivation rule is an **internal sub-step** of `resolve_action`:

```rust
resolve_action(p: Progression, c: Capacity, a: Action)
  -> (Progression, Capacity, Option<IZ>);
```

`resolve_action` remains the **sole observable boundary**.

---

## 8. Forbidden Surfaces (Strict)

Presence of any of the following invalidates the system:

- ❌ Alternative derivation algorithms
- ❌ Runtime selection of hash functions
- ❌ Mutable or extensible `AdmissibleSet`
- ❌ Semantic decoding of `Action`
- ❌ Time-based derivation rules
- ❌ Side-channel leakage (timing, power, etc.)
- ❌ Any conversion or reuse of `IZ` as capability
- ❌ Version negotiation of derivation logic

---

*Document canonique — Phase 4.3 Internal Derivation*  
*MN-001-SYFCORE-20260201-AB-P4.3-SEALED*
