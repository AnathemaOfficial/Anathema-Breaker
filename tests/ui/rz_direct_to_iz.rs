// Compile-fail test: RZ cannot transition directly to IZ.
// Must go through EP first (RZ → EP → IZ).

use anathema_breaker_core::pom::topology::{Action, RZ};
use anathema_breaker_core::pom::types::{Domain, Magnitude};

fn main() {
    let a = Action::<RZ>::new(Domain(1), Magnitude(1));
    let _ = a.into_iz(); // ERROR: no method `into_iz` for Action<RZ>
}
