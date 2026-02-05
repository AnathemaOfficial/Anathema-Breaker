//! Minimal integration example — AB branché sans altérer la loi.
//! Demonstrates: zero semantic understanding required by AB.

// ──────────────────────────────────────────────────────────────
// Système externe fictif (ex: API, DB, payment gateway)
// ──────────────────────────────────────────────────────────────

use anathema_breaker_core::pom::resolve_action::resolve_action;
use anathema_breaker_core::pom::topology::{Action, RZ};
use anathema_breaker_core::pom::types::*;

struct ExternalSystem {
    budget_capacity: u32,
    budget_progression: u32,
}

impl ExternalSystem {
    /// Branchement zéro-sémantique sur AB
    /// AB ne comprend PAS "payment" ou "DB write" — uniquement Domain + Magnitude
    fn handle_request(&mut self, domain_id: u16, magnitude: u32) {
        let action = Action::<RZ>::new(Domain(domain_id), Magnitude(magnitude));
        let mut budget = Budget {
            capacity: Capacity(self.budget_capacity),
            progression: Progression(self.budget_progression),
        };
        let inv = Invariants {
            r: 1,
            flow: 2,
            entropy: 3,
        };

        // ──── Pattern d'intégration canonique ────
        // OK: effet produit → exécuter logique métier
        // Err: impossibilité structurelle → SILENCE (pas de signal exploitable)
        match resolve_action(action, &mut budget, inv) {
            Ok(effect) => {
                // Mise à jour du budget local (effet consommé)
                self.budget_capacity = budget.capacity.0;
                self.budget_progression = budget.progression.0;
                self.execute_business_logic(effect.magnitude_applied.0);
            }
            Err(_) => {
                // FAIL-CLOSED ABSOLU
                // Aucun retour d'information exploitable vers l'appelant
                // (ex: même code HTTP 200 avec payload vide, pas 429/403)
                self.deny_without_feedback();
            }
        }
    }

    fn execute_business_logic(&mut self, _magnitude: u32) {
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
        budget_capacity: 100,
        budget_progression: 10,
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
            budget_capacity: 100,
            budget_progression: 10,
        };
        system.handle_request(1, 50); // autorisé → exécution
        system.handle_request(1, 200); // impossibilité → silence
    }
}
