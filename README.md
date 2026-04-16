
<img width="256" height="256" alt="SYF Core copy" src="https://github.com/user-attachments/assets/d3e162c8-427c-4045-ba7d-5a15f3184792" />

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

## Historical Documents

- `README_PHASE5.md` - archived Phase 5.1 structure-only note
- `README_P7.md` - current AB-S sealing note

## Verification

```bash
cargo test
cargo test --test compile_tests
cargo run --example p7_sealed_machine
```
