#[cfg(test)]
mod tests {
    use crate::game::character::AbilityScores;
    use crate::game::character::stats::SavingThrowProficiencies;
    use crate::game::character::Character;
    use crate::game::character::skills::Skill;
    use crate::game::character::conditions::Condition;
    fn test_char() -> Character {
        Character::new(
            "Theron".into(),
            "fighter".into(),
            "human".into(),
            AbilityScores {
                strength: 16,
                dexterity: 14,
                constitution: 14,
                intelligence: 10,
                wisdom: 12,
                charisma: 8,
            },
        )
    }

    #[test]
    fn modifier_formula() {
        assert_eq!(AbilityScores::modifier(10), 0);
        assert_eq!(AbilityScores::modifier(12), 1);
        assert_eq!(AbilityScores::modifier(8), -1);
        assert_eq!(AbilityScores::modifier(20), 5);
    }

    #[test]
    fn take_damage_temp_hp() {
        let mut c = test_char();
        c.temp_hp = 5;
        c.take_damage(3);
        assert_eq!(c.temp_hp, 2);
        assert_eq!(c.current_hp, c.max_hp);
    }

    #[test]
    fn take_damage_to_zero_adds_unconscious() {
        let mut c = test_char();
        c.take_damage(c.current_hp as u32 + 10);
        assert_eq!(c.current_hp, 0);
        assert!(c.conditions.contains(&Condition::Unconscious));
    }

    #[test]
    fn heal_removes_unconscious() {
        let mut c = test_char();
        c.conditions.insert(Condition::Unconscious);
        c.heal(1);
        assert!(!c.conditions.contains(&Condition::Unconscious));
    }
}