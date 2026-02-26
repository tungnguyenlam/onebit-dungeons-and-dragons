use super::WorldState;

#[derive(Debug, Clone)]
pub struct EndingResult {
    pub title: String,
    pub body: String,
    pub score: i32,
}

pub fn calculate(world: &WorldState) -> EndingResult {
    let mut score = 0;
    if world.flag("drow_allied") {
        score += 2;
    }
    if world.flag("ritual_completed") {
        score += 1;
    }
    if world.flag("silence_heard") {
        score += 1;
    }
    if world.flag("silence_silenced") {
        score += 2;
    }
    if world.counter("faction_town_guard_rep") > 0 {
        score += 1;
    }
    if world.counter("faction_goblin_tribe_rep") > 0 {
        score += 1;
    }

    if score >= 5 {
        EndingResult {
            title: "Dawn Over the Rift".into(),
            body: "You defeat the Void Architect and return as a unifier. Alliances endure, the rifts calm, and a fragile new peace takes hold.".into(),
            score,
        }
    } else if score >= 3 {
        EndingResult {
            title: "The Quiet Scar".into(),
            body: "The final battle is won, but the world bears deep scars. Kingdoms survive, yet trust remains brittle and the abyss is only sleeping.".into(),
            score,
        }
    } else {
        EndingResult {
            title: "Ashen Victory".into(),
            body: "You silence the abyss by force. Civilization endures, but at a heavy cost. Fear spreads faster than hope in the years that follow.".into(),
            score,
        }
    }
}

pub fn credits_lines() -> Vec<String> {
    vec![
        "OneBit Dungeons & Dragons".into(),
        "".into(),
        "Design & Engineering: OneBit Agents".into(),
        "Narrative Systems: World Simulation Team".into(),
        "Combat & Progression: Rules Team".into(),
        "Content & Regions: Quest Team".into(),
        "".into(),
        "Thank you for playing.".into(),
        "Press Esc to return to Main Menu.".into(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heroic_ending_when_score_high() {
        let mut ws = WorldState::new();
        ws.set_flag("drow_allied");
        ws.set_flag("ritual_completed");
        ws.set_flag("silence_silenced");
        ws.set_counter("faction_town_guard_rep", 2);
        let out = calculate(&ws);
        assert!(out.score >= 5);
        assert_eq!(out.title, "Dawn Over the Rift");
    }
}
