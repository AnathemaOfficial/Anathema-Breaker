<img width="256" height="256" alt="SYF Core copy" src="https://github.com/user-attachments/assets/beb60181-4f66-4090-97ea-80d776a9318e" />

# Anathema-Breaker

**Current status:** `Phase 7.0 / AB-S`  
**Scope:** formally sealed software machine with representational impossibility

Anathema-Breaker is no longer a Phase 5.1 structure-only skeleton.
The active crate now exposes the sealed `AB-S` software tier built from the
Phase 6.2 PoM core and sealed in Phase 7.0.

The operative truth is:

- typestate topology `RZ -> EP -> IZ`
- deterministic `resolve_action`
- compile-stage path absence for invalid transitions
- no governance, no time, no feedback, no optimization

This repository should be read in the following order:

1. `specs/CANON_INDEX.md`
2. `README_P7.md`
3. `src/pom/`

Historical phase notes remain in the repository for traceability, but they are
not the active product or machine description.

## Canonical Lineage

Anathema-Breaker is the **sealed synthesis of Gate and Shield** into a resolution core.
It is neither the upstream law nor the downstream execution membrane.

```
SYF-Core  →  SYF-Gate  →  SYF-Shield  →  Anathema-Breaker  →  SLIME-Core
  theory     admission     capacity        resolution core      membrane
```

- **SYF-Core** — upstream thermodynamic theory (`R = (F × E) / K`)
- **SYF-Gate** — structural admissibility primitive
- **SYF-Shield** — capacity, progression, and irreversibility primitive
- **Anathema-Breaker (this repo)** — sealed synthesis of Gate + Shield
- **SLIME-Core** — canonical execution membrane derived from AB

**AB is the heart.** **SLIME-Core is the canonical execution membrane built from it.**
SLIME-Core is a downstream derivation, not an upstream dependency.

## Historical Documents

- `README_PHASE5.md` - archived Phase 5.1 structure-only note
- `README_P7.md` - current AB-S sealing note

## Verification

```bash
cargo test
cargo test --test compile_tests
cargo run --example p7_sealed_machine
```
