// Compile-fail test: Budget fields cannot be mutated externally.
// SYMBIOTE falsification: "budget cheat makes impossible become possible"
//
// Before this fix, an integrator could do:
//   budget.capacity.0 += 1000;
// and turn an IMPOSSIBLE into AUTHORIZED without touching AB-S.
//
// After this fix, Budget/Capacity/Progression fields are private.
// The only mutation path is through resolve_action (crate-internal).

use anathema_breaker_core::pom::types::{Budget, Capacity, Progression};

fn main() {
    let mut budget = Budget::new(Capacity::new(10), Progression::new(1));

    // Attempt to cheat: directly mutate capacity field
    // This MUST fail to compile — field `capacity` is private
    budget.capacity = Capacity::new(9999);
    //~^ ERROR field `capacity` of struct `Budget` is private
}
