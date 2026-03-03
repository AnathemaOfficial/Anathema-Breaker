// Compile-fail test: EP cannot transition back to RZ.
// Engagement is irreversible (EP → IZ only).

use anathema_breaker_core::pom::topology::{Action, RZ};
use anathema_breaker_core::pom::types::{Domain, Magnitude};

fn main() {
    let ep = Action::<RZ>::new(Domain::new(1), Magnitude::new(1)).engage();
    let _ = ep.back_to_rz(); // ERROR: no method `back_to_rz` for Action<EP>
}
