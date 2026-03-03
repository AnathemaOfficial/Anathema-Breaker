//! P7-S1: Effect cannot flow back to Action (feedback impossibility)
//!
//! This test proves that AB-S provides no mechanism for effects to influence
//! future action resolutions. The type system makes feedback paths unrepresentable.

use anathema_breaker_core::pom::topology::{Action, RZ};
use anathema_breaker_core::pom::types::{Budget, Capacity, Domain, Effect, Magnitude, Progression};
use anathema_breaker_core::pom::resolve_action::resolve_action;

fn main() {
    // Resolve an action to produce an Effect
    let action = Action::<RZ>::new(Domain::new(1), Magnitude::new(100));
    let mut budget = Budget::new(Capacity::new(1000), Progression::new(10));
    let effect: Effect = resolve_action(action, &mut budget).unwrap();

    // Attempt to convert Effect back to Action (feedback path)
    // This MUST fail to compile — no API exists for this conversion
    let _action_back: Action<RZ> = effect.into_action();
    //~^ ERROR no method named `into_action` found for struct `Effect`
}
