#[cfg(test)]
mod tests {
    use super::*;
    use super::super::utils::*;

    #[test]
    fn summary_prefers_high_signal_events() {
        let log = vec![
            "Goblin now has 3 HP.".to_string(),
            "Player hits Goblin for 7 damage (d20=15 total=19).".to_string(),
        ];
        assert!(last_turn_summary(&log).contains("hits Goblin"));
    }

    fn last_turn_summary(log: &[String]) -> String {
        for line in log.iter().rev() {
            if line.contains("CRITS")
                || line.contains("critical")
                || line.contains("drops to 0 HP")
                || line.contains("hits")
                || line.contains("miss")
                || line.contains("recovers")
                || line.contains("restoring")
                || line.contains("expired")
            {
                return line.clone();
            }
        }
        "No major action yet.".to_string()
    }
}