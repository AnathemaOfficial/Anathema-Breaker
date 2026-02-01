# CANON_INDEX.md
## Anathema-Breaker — Canonical Index & Law Map
*Authoritative Reference Document*  
*Status: CANON / SEALED*

---

## 0. Purpose

This document is the **single authoritative index** for the Anathema-Breaker canon.

It:

* enumerates all **sealed canonical documents**
* defines their **scope, authority, and dependencies**
* establishes the **non-negotiable laws** governing Anathema-Breaker
* prevents drift, reinterpretation, or partial adoption

If a statement is not derivable from the documents listed here, it is **non-canonical**.

---

## 1. Canonical Definition (One Sentence)

> **Anathema-Breaker is a finite SLIME cluster that makes certain actions structurally impossible by geometry, thermodynamic accounting, and path absence — independent of intelligence, intent, or governance.**

---

## 2. Canonical Layers Overview

```
┌──────────────────────────────┐
│ README v0.1                  │  ← Product / concept boundary
├──────────────────────────────┤
│ Phase 3 — Minimal Types      │  ← Geometry & signatures
├──────────────────────────────┤
│ Phase 4.1 — Action Envelope  │  ← Input law
├──────────────────────────────┤
│ Phase 4.2 — IZ Envelope      │  ← Output law
├──────────────────────────────┤
│ Phase 4.3 — Derivation Rule  │  ← Internal law
└──────────────────────────────┘
```

Each layer is **sealed independently** and **composable only downward**.
No layer may redefine or override an upstream law.

---

## 3. Canonical Documents (Sealed)

| Layer | File | Scope | Status |
|-------|------|-------|--------|
| **v0.1** | `README.md` | Product definition: chamber of impossibility | SEALED |
| **Phase 3** | `SPEC_PHASE3_MINIMAL_TYPES.md` | Abstract types + `resolve_action` | SEALED |
| **Phase 4.1** | `SPEC_PHASE4_1_ACTION_OPAQUE.md` | Input envelope `Action([u8;32])` | SEALED |
| **Phase 4.2** | `SPEC_PHASE4_2_IZ_ENVELOPE.md` | Output envelope `IZ([u8;32])` + dissipation | SEALED |
| **Phase 4.3** | `SPEC_PHASE4_3_DERIVATION_RULE.md` | Internal derivation `Action → IZ` | SEALED |

All documents are immutable once sealed.
Any modification constitutes a **fork**, not an update.

---

## 4. Canonical Lexicon (Non-Extensible)

Only the following terms are valid in canonical context:

| Term | Meaning |
|------|---------|
| `SLIME` | Law-invariant systemic milieu |
| `FiniteCluster` | Bounded SLIME volume |
| `Progression` | `{ RZ → EP → IZ }` topological position |
| `RZ` | Reversible Zone |
| `EP` | Engagement Point (first irreversibility) |
| `IZ` | Irreversible Zone (terminal) |
| `Action` | Opaque input envelope `[u8;32]` |
| `IZ([u8;32])` | Output envelope emitted from IZ domain |
| `Capacity` | Opaque thermodynamic accounting unit |
| `δ` | Internal, opaque thermodynamic consumption |
| `ShieldTopology` | Deployment-declared routing geometry |
| `TopologyID` | Immutable identifier derived at deployment |

Any additional term (e.g. *state*, *policy*, *intent*, *command*) is **non-canonical**.

---

## 5. Canonical Invariants (Global Laws)

These laws apply **across all phases**:

1. **No Semantics**
   No interpretation of meaning, intent, or goals.

2. **Temporal Ban**
   No time, clocks, TTLs, expiries, freshness, or sequencing.

3. **Path Absence**
   Bypass paths are not forbidden — they are **undefined**.

4. **Fail-Closed**
   Insufficiency yields no actuation and no corruption.

5. **Thermodynamic Accounting**
   Successful actuation implies δ > 0 consumption (opaque).

6. **Topology Immutability**
   Geometry is fixed at deployment; no runtime mutation.

7. **Dissipation**
   Outputs from domain `IZ` leave the cluster and cannot return.

Violation of any invariant = **non-Breaker system**.

---

## 6. Boundary of Authority

| Layer | Allowed to Define | Forbidden to Define |
|-------|-------------------|---------------------|
| README | What AB *is* | How it computes |
| Phase 3 | Types, signatures | Envelope formats |
| Phase 4.1 | Action envelope | Semantics, commands |
| Phase 4.2 | IZ envelope | Feedback channels |
| Phase 4.3 | Derivation law | Alternative algorithms |

No layer may:

* introduce configuration knobs
* introduce governance
* introduce human override
* reinterpret upstream constraints

---

## 7. Non-Goals (Explicit)

Anathema-Breaker does **not**:

* align intelligence
* enforce policy
* interpret behavior
* optimize outcomes
* learn or adapt
* provide admin modes
* expose configuration surfaces
* rely on cryptographic trust assumptions

Any system claiming these properties is **not Anathema-Breaker**.

---

## 8. Canonical Dependency Graph

```
README v0.1
   ↓
Phase 3 (Types & Geometry)
   ↓
Phase 4.1 (Action Input Law)
   ↓
Phase 4.3 (Derivation Law)
   ↓
Phase 4.2 (IZ Output Law & Dissipation)
```

Note:
Phase 4.3 is **internal** and never observable externally.
Phase 4.2 defines the **only observable output**.

---

## 9. Seal

```
ANATHEMA-BREAKER — CANON INDEX
Authority: Systemic Law
Status: SEALED
Hash: MN-001-SYFCORE-20260201-AB-CANON-INDEX
```

---

## 10. Final Statement

> **Controls can be bypassed.
> Policies can be negotiated.
> Laws cannot.**

> **Anathema-Breaker is not a control system.
> It is a finite geometry where certain actions do not exist.**

---

*Document canonique — Canon Index*  
*MN-001-SYFCORE-20260201-AB-CANON-INDEX*
