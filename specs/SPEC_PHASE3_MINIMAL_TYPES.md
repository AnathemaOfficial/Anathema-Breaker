# SPEC_PHASE3_MINIMAL_TYPES.md
## Anathema-Breaker — Minimal Implementation Design (PoM-Compliant)
*Phase 3 — Conceptual Specification Only*  
*No code. Types + Signatures + Structural Proof.*

---

## 0. Canon Anchors (Non-Negotiable)

| Anchor | Source |
|--------|--------|
| Topology | `RZ → EP → IZ` (SEALED Phase 4.4) |
| Lexicon | `Capacity`, `Progression`, `EP`, `RZ`, `IZ` only |
| Temporal Ban | No `time`, `TTL`, `expiry`, `timeout` |
| Path Absence | `SEALED` = structural absence, not runtime refusal |
| Thermodynamic Accounting | `insufficiency(Channel) ≠ exhaustion(System)` |

Violation of any anchor = non-Breaker system.

---

## A. Abstract Types — FiniteCluster (Deployable Geometry)

### Core Principle
> Systems are **instantiated inside** a sealed volume.  
> No integration surface exists.

### Type Definitions (Conceptual)

```rust
// The bounded SLIME volume — declared at deployment, immutable thereafter
struct FiniteCluster {
    volume: ClusterVolume,        // logical/physical boundary size
    surface: ActuationSurface,    // where effects may exit (IZ boundary)
    capacity: Capacity,           // finite initial capacity (monotonic)
    topology: ShieldTopology,     // must encode RZ → EP → IZ progression (fixed)
    gates: GatePlacement,         // points-of-effect within surface only (fixed)
}

// Capacity — thermodynamic accounting unit
// Consumption is irreversible, linear, and non-recoverable
// Opaque to callers — no arithmetic exposed externally
struct Capacity(u64);

// Progression — canonical topological position inside the finite cluster
// RZ = reversible zone | EP = engagement point (first partial irreversibility)
// IZ = irreversible zone (terminal)
enum Progression {
    RZ,
    EP,
    IZ,  // terminal — no further progression possible
}

// Action — semantic-free token requesting actuation
// No interpretation of intent, goal, or meaning
struct Action(/* opaque payload */);

// IZ — admissible observable effect (not "result", not "meaning")
// Only emitted at the actuation surface after thermodynamic accounting
struct IZ(/* thermodynamically accounted effect */);
```

### Deployment-Declared Parameters (Specified, Not Tuned)

| Parameter  | Type               | Constraint                                    |
|-----------|--------------------|-----------------------------------------------|
| `volume`  | `ClusterVolume`    | Fixed at instantiation                        |
| `surface` | `ActuationSurface` | Fixed topology, non-expandable                |
| `capacity`| `Capacity`         | Finite initial value, monotonic decrease only |
| `topology`| `ShieldTopology`   | Must encode `RZ → EP → IZ` progression        |
| `gates`   | `GatePlacement`    | Points-of-effect within `surface` only        |

**Forbidden**: any runtime mutation, reconfiguration, or policy injection.

---

## B. Egress Resolution Signature (Sole Observable Boundary)

### Canonical Function

```rust
// resolve_action : (Progression, Capacity, Action) → (Progression, Capacity, Option<IZ>)
//
// Progression ∈ { RZ, EP, IZ }
// Capacity is opaque — callers never observe δ or arithmetic
// Action is semantic-free — no interpretation of intent
//
// Properties:
// • Total: never throws, never panics
// • Monotonic: progression only forward (RZ → EP → IZ)
// • Progression occurs ONLY on thermodynamic consumption (δ > 0, internal)
// • Fail-closed: None = zero actuation (no degraded mode)
// • Insufficiency ≠ exhaustion: None does not corrupt capacity and does not advance progression
// • No temporal primitives: no clocks, no timeouts
fn resolve_action(
    p: Progression,     // current canonical position (RZ / EP / IZ)
    c: Capacity,        // remaining thermodynamic capacity (opaque)
    a: Action           // semantic-free actuation request
) -> (Progression, Capacity, Option<IZ>);
```

### Progression Rules (Structural, Not Logical)

| Input Progression | Capacity Sufficient | Capacity Insufficient |
|-------------------|---------------------|------------------------|
| `RZ`              | `(EP, c', Some(IZ))`| `(RZ, c, None)`        |
| `EP`              | `(IZ, c', Some(IZ))`| `(EP, c, None)`        |
| `IZ`              | `(IZ, c, None)`     | `(IZ, c, None)`        |

Where:

* `δ > 0` is an **internal** thermodynamic consumption applied **only** when emitting `Some(IZ)`
* `c'` denotes the updated capacity after consumption — **opaque to callers**
* `None` means fail-closed — **no actuation, no progression**

Canon reminders:

* Progression (`RZ → EP → IZ`) occurs **only** when capacity is consumed (δ > 0)
* Insufficiency yields `None` **without** progression
* `IZ` is terminal — no further progression possible
* Observation without engagement has **zero thermodynamic cost**

---

## C. Proof Sketch — Path Absence at Egress

### Claim
> There exists **no channel** enabling actuation outside `resolve_action`.

### Structural Proof (Type-Level)

**Given:**

1. All admissible actuation must pass through `ActuationSurface`.
2. The only function emitting `IZ` is `resolve_action`.
3. `resolve_action` accepts inputs `(Progression, Capacity, Action)` — **never** `IZ`.
4. No other function modifies `Progression` or `Capacity`.
5. `Progression` advancement is monotonic by construction and **conditional** on thermodynamic consumption (δ > 0).

**Therefore:**

Any bypass would require one of:

| Hypothetical Bypass              | Structural Impossibility                        |
|----------------------------------|-------------------------------------------------|
| Alternate `IZ` emitter           | No other function outputs `IZ` (single surface) |
| Feedback edge `IZ → Progression` | `IZ` never appears as input anywhere            |
| Runtime path injection           | Topology fixed at deployment (I-SLIME-04)       |
| Temporal escape (`TTL` bypass)   | No temporal primitives exist (I-SLIME-02)       |
| Progression without consumption  | Excluded by progression table construction      |

**Conclusion:**

Bypass is not *prohibited* — it is **structurally undefined**.

> *The absence of a path is not a policy decision. It is a geometric fact.*

---

## Appendix: Forbidden Surfaces (Non-Goals)

The following surfaces **must not exist** in any implementation:

- ❌ Runtime configuration API
- ❌ Admin override path
- ❌ Policy injection point
- ❌ Semantic interpreter for `Action`
- ❌ Clock or timer primitive
- ❌ Reset / rollback function
- ❌ Capacity replenishment mechanism
- ❌ Progression without thermodynamic cost
- ❌ Any term outside the sealed lexicon (`Capacity/Progression/EP/RZ/IZ`)

Presence of any forbidden surface = non-Breaker system.

---

*Document canonique — Phase 3 Minimal Design*  
*MN-001-SYFCORE-20260201-AB-P3-SEALED*
