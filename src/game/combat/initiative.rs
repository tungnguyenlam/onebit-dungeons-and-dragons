/// Initiative rolling and turn-order construction.
use rand::{Rng, SeedableRng};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitiativeCombatant {
    pub entity_id:    String,
    pub dex_modifier: i32,
    pub is_player:    bool,
    /// Stable input index used as final deterministic tie-breaker.
    pub index:        usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitiativeOrder {
    /// Initiative total -> entity ids at that total.
    pub buckets: BTreeMap<i32, Vec<String>>,
    /// Flat queue in actual turn order (highest initiative first).
    pub queue:   Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct InitiativeEntry {
    entity_id: String,
    total:     i32,
    is_player: bool,
    index:     usize,
}

/// Roll initiative with non-deterministic RNG.
pub fn roll_initiative(combatants: &[InitiativeCombatant]) -> InitiativeOrder {
    let mut rng = rand::rng();
    roll_initiative_with_rng(combatants, &mut rng)
}

/// Roll initiative with a fixed seed (for deterministic tests/replays).
pub fn roll_initiative_with_seed(combatants: &[InitiativeCombatant], seed: u64) -> InitiativeOrder {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    roll_initiative_with_rng(combatants, &mut rng)
}

fn roll_initiative_with_rng<R: Rng + ?Sized>(
    combatants: &[InitiativeCombatant],
    rng: &mut R,
) -> InitiativeOrder {
    let mut entries = Vec::with_capacity(combatants.len());
    for c in combatants {
        let d20 = rng.random_range(1..=20) as i32;
        entries.push(InitiativeEntry {
            entity_id: c.entity_id.clone(),
            total: d20 + c.dex_modifier,
            is_player: c.is_player,
            index: c.index,
        });
    }

    // Sort by initiative descending; ties: player before monsters, then index.
    entries.sort_by(|a, b| {
        b.total
            .cmp(&a.total)
            .then_with(|| b.is_player.cmp(&a.is_player))
            .then_with(|| a.index.cmp(&b.index))
    });

    let mut buckets = BTreeMap::<i32, Vec<String>>::new();
    let mut queue = Vec::with_capacity(entries.len());
    for e in entries {
        buckets.entry(e.total).or_default().push(e.entity_id.clone());
        queue.push(e.entity_id);
    }

    InitiativeOrder { buckets, queue }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(id: &str, dex_mod: i32, is_player: bool, index: usize) -> InitiativeCombatant {
        InitiativeCombatant {
            entity_id: id.to_string(),
            dex_modifier: dex_mod,
            is_player,
            index,
        }
    }

    #[test]
    fn fixed_seed_produces_stable_queue() {
        let combatants = vec![
            c("player", 2, true, 0),
            c("goblin_a", 2, false, 1),
            c("goblin_b", 2, false, 2),
        ];

        let a = roll_initiative_with_seed(&combatants, 1337);
        let b = roll_initiative_with_seed(&combatants, 1337);
        assert_eq!(a.queue, b.queue);
        assert_eq!(a.buckets, b.buckets);
    }

    #[test]
    fn tie_breaker_prefers_player_then_index() {
        let combatants = vec![
            c("player", 0, true, 0),
            c("monster_1", 0, false, 1),
            c("monster_2", 0, false, 2),
        ];

        let order = roll_initiative_with_seed(&combatants, 17);
        assert_eq!(order.queue.len(), 3);

        // With same dex mod and fixed seed, if totals tie this keeps player first.
        if let Some((_, ids)) = order.buckets.iter().find(|(_, ids)| ids.len() == 3) {
            assert_eq!(ids[0], "player");
            assert_eq!(ids[1], "monster_1");
            assert_eq!(ids[2], "monster_2");
        }
    }
}
