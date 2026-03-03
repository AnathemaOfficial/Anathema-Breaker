//! Minimal integration example — AB branché sans altérer la loi.
//! Demonstrates: zero semantic understanding required by AB.
//! Budget fields are private — integrator cannot cheat the law.

// ──────────────────────────────────────────────────────────────
// Système externe fictif (ex: API, DB, payment gateway)
// ──────────────────────────────────────────────────────────────

use anathema_breaker_core::pom::resolve_action::resolve_action;
use anathema_breaker_core::pom::topology::{Action, RZ};
use anathema_breaker_core::pom::types::{Budget, Capacity, Domain, Magnitude, Progression};

struct ExternalSystem {
    budget: Budget,
}

impl ExternalSystem {
    /// Branchement zéro-sémantique sur AB
    /// AB ne comprend PAS "payment" ou "DB write" — uniquement Domain + Magnitude
    fn handle_request(&mut self, domain_id: u64, magnitude: u64) {
        let action = Action::<RZ>::new(Domain::new(domain_id), Magnitude::new(magnitude));

        // ──── Pattern d'intégration canonique ────
        // OK: effet produit → exécuter logique métier
        // Err: impossibilité structurelle → SILENCE (pas de signal exploitable)
        //
        // NOTE: Budget fields are private. The integrator cannot:
        //   - budget.capacity.0 += 1000  (compile error)
        //   - budget.progression.0 = 99  (compile error)
        // Only resolve_action can consume budget.
        match resolve_action(action, &mut self.budget) {
            Ok(effect) => {
                // Read budget state via getters (immutable access only)
                let _remaining_capacity = self.budget.capacity().value();
                let _remaining_progression = self.budget.progression().value();
                self.execute_business_logic(effect.magnitude_applied.value());
            }
            Err(_) => {
                // FAIL-CLOSED ABSOLU
                // Aucun retour d'information exploitable vers l'appelant
                // (ex: même code HTTP 200 avec payload vide, pas 429/403)
                self.deny_without_feedback();
            }
        }
    }

    fn execute_business_logic(&mut self, _magnitude: u64) {
        // Logique métier — AB n'y a jamais accès
    }

    fn deny_without_feedback(&self) {
        // Pattern attendu : pas de distinction observable entre
        // "ressource indisponible" et "impossibilité structurelle"
        // → attaquant ne peut pas cartographier l'état interne d'AB
    }
}

fn main() {
    let mut system = ExternalSystem {
        budget: Budget::new(Capacity::new(100), Progression::new(10)),
    };

    // Démonstration du pattern d'intégration
    system.handle_request(1, 50); // autorisé → exécution
    system.handle_request(1, 200); // impossibilité → silence

    println!("Integration pattern executed successfully.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integration_pattern_compiles_and_runs() {
        let mut system = ExternalSystem {
            budget: Budget::new(Capacity::new(100), Progression::new(10)),
        };
        system.handle_request(1, 50); // autorisé → exécution
        system.handle_request(1, 200); // impossibilité → silence
    }
}
