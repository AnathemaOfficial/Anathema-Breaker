# Anathema-Breaker Tier System

**Document Type:** Non-Canonical (Guidance)  
**Last Updated:** 2026-02-05  
**Status:** Living document (subject to updates as tiers evolve)

---

## Overview

Anathema-Breaker's security guarantees are organized into **tiers** representing
progressively stronger impossibility claims.

Each tier is **explicitly scoped** to avoid overpromising. Higher tiers build on
lower tiers but require additional hardware or physical infrastructure.

---

## Tier Definitions

### **AB-S (Software Tier)** — Current

**Status:** ✅ Phase 7.0 Sealed  
**Impossibility Type:** Representational (type system)  
**Hardware Requirement:** Standard computer (any OS, any architecture)

#### What AB-S Guarantees

- ✅ **Compile-stage path absence:** Invalid topologies cannot be expressed in safe Rust
- ✅ **Deterministic resolution:** Same inputs → same outputs (no hidden state)
- ✅ **No feedback channels:** Effects cannot influence future action resolutions
- ✅ **Immutable law:** Budget accounting rules are fixed and auditable

#### What AB-S Does NOT Guarantee

- ❌ Physical impossibility (code runs on general-purpose hardware)
- ❌ Protection against host environment compromise (OS, memory inspection, etc.)
- ❌ Defense against side-channel attacks (timing, power analysis, etc.)

#### Use Cases

- Formal verification prototypes
- Software-defined critical systems
- Academic research / proof-of-concept
- Integration testing for future hardware tiers

#### Deployment Example

```rust
// AB-S runs as a normal Rust library
use anathema_breaker_core::pom::{resolve_action, Budget, Action};

let mut budget = Budget::new(1000, 10);
let action = Action::new(domain, magnitude);
match resolve_action(action, &mut budget) {
    Ok(effect) => { /* actuation */ },
    Err(_) => { /* structural impossibility */ },
}
```

---

### **AB-H (Hardware Proto Tier)** — Future (Q3-Q4 2026)

**Status:** ⏳ Research Phase  
**Impossibility Type:** Structural (reduced attack surface)  
**Hardware Requirement:** Embedded systems (microcontroller, FPGA)

#### What AB-H Will Add

- ✅ **No operating system:** Bare-metal deployment eliminates OS attack surface
- ✅ **Unidirectional I/O:** Physical buses with no return path (hardware-enforced)
- ✅ **Minimal trusted computing base:** Only bootloader + AB kernel (no userland)
- ✅ **Side-channel hardening:** Constant-time operations, power filtering

#### What AB-H Will NOT Guarantee

- ❌ Thermodynamic impossibility (still electronic circuits, modifiable with tools)
- ❌ Protection against physical tampering (chip decapping, probing, etc.)
- ❌ Scalability to large systems (embedded constraints)

#### Candidate Platforms

- ARM Cortex-M (STM32, nRF52)
- RISC-V bare-metal (ESP32-C3, SiFive)
- FPGA (Lattice iCE40, Xilinx)

---

### **AB-R (ROCHE Tier)** — Research (2027+)

**Status:** 🔬 Experimental (requires La ROCHE device)  
**Impossibility Type:** Thermodynamic (physical constraints)  
**Hardware Requirement:** Custom thermodynamic machine (La ROCHE)

#### What AB-R Aims to Provide

- ✅ **Physical impossibility:** Constraints enforced by material properties, not code
- ✅ **Thermodynamic excitation:** Energy input that cannot be interpreted as information
- ✅ **Topological boundary:** Physical membrane where possibility ceases
- ✅ **No software-exploitable paths:** Even with complete knowledge of design

#### What AB-R Cannot Guarantee

- ❌ Protection against destruction (thermodynamic machines can be destroyed)
- ❌ Scalability (each instance requires custom fabrication)
- ❌ Adaptability (law is physically sealed — no software updates possible)

#### La ROCHE Concept

La ROCHE is a thermodynamic device that:
- Receives **noise** (non-interpretable energy gradient) as input
- Enforces **topological confinement** through physical structure
- Produces **effects** with no information channel back to the machine

This is the ultimate realization of SYF principles: impossibility as physics, not policy.

---

## Tier Migration Path

```
Phase 6.2 (Mechanism)
    ↓
Phase 7.0 (AB-S sealed)
    ↓
Phase 7.1 (AB-H prototyping)
    ↓
Phase 7.2 (AB-H production)
    ↓
Phase 7.x (AB-R research)
```

Each tier **preserves the law** defined in earlier phases. Migration is about
**enforcement strength**, not law modification.

---

## Choosing the Right Tier

| Your Requirement | Recommended Tier |
|------------------|------------------|
| Formal verification, academic research | **AB-S** (current) |
| Critical embedded systems (medical, automotive) | **AB-H** (future) |
| Highest-stakes autonomous systems (no human fallback) | **AB-R** (research) |
| Software-only deployment | **AB-S** only option |
| Need for software updates | **AB-S** or **AB-H** (AB-R cannot be updated) |
| Must resist physical tampering | Only **AB-R** (others vulnerable) |

---

## Tier Honesty Principle

Each tier **explicitly declares** what it guarantees and what it does not.

We do not claim:
- AB-S provides physical impossibility (it does not)
- AB-H is immune to physical attacks (it is not)
- AB-R is indestructible (it is not)

We claim only:
- AB-S: representational impossibility (type system)
- AB-H: structural impossibility (hardware architecture)
- AB-R: thermodynamic impossibility (physics)

**Honesty about limitations is a feature, not a bug.**

---

## References

- **Phase 7.0 Spec:** `specs/MN-001-P7-MACHINE-SEALING.md` (canonical)
- **SYF-Core:** Foundational thermodynamic law (separate repository)
- **SLIME Concept:** `docs/SLIME-vs-AB.md` (AB as local instantiation of SLIME DNA)

---

**Last Updated:** 2026-02-05  
**Maintainer:** SYFCORP / Anathema Project  
**License:** Same as Anathema-Breaker core (Apache-2.0)
