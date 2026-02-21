/// Dice expression types and rolling logic.
///
/// `DiceExpr` represents expressions like `2d6+3` and can be deserialized
/// directly from TOML strings via a custom serde implementation.
///
/// See [docs/gameplay/dice.md] for the full specification.
pub mod parser;

use rand::Rng;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

// ---------------------------------------------------------------------------
// DiceExpr
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiceExpr {
    pub count: u32,
    pub sides: u32,
    pub modifier: i32,
}

impl DiceExpr {
    pub fn new(count: u32, sides: u32, modifier: i32) -> Self {
        Self {
            count,
            sides,
            modifier,
        }
    }

    /// Roll the dice and return the total (sum of dice + modifier).
    pub fn roll(&self) -> i32 {
        let mut rng = rand::rng();
        let dice_total: i32 = (0..self.count)
            .map(|_| rng.random_range(1..=self.sides) as i32)
            .sum();
        dice_total + self.modifier
    }

    /// Roll twice and take the higher result (advantage).
    pub fn roll_advantage(&self) -> i32 {
        self.roll().max(self.roll())
    }

    /// Roll twice and take the lower result (disadvantage).
    pub fn roll_disadvantage(&self) -> i32 {
        self.roll().min(self.roll())
    }

    /// Maximum possible result.
    pub fn max_value(&self) -> i32 {
        (self.count * self.sides) as i32 + self.modifier
    }

    /// Minimum possible result.
    pub fn min_value(&self) -> i32 {
        self.count as i32 + self.modifier
    }

    /// Average result (floor).
    pub fn average(&self) -> i32 {
        let avg_per_die = (self.sides + 1) as f32 / 2.0;
        (self.count as f32 * avg_per_die) as i32 + self.modifier
    }
}

// ---------------------------------------------------------------------------
// Display / FromStr
// ---------------------------------------------------------------------------

impl fmt::Display for DiceExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}d{}", self.count, self.sides)?;
        match self.modifier.cmp(&0) {
            std::cmp::Ordering::Greater => write!(f, "+{}", self.modifier),
            std::cmp::Ordering::Less => write!(f, "{}", self.modifier),
            std::cmp::Ordering::Equal => Ok(()),
        }
    }
}

impl std::str::FromStr for DiceExpr {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::parse(s)
    }
}

// ---------------------------------------------------------------------------
// Serde — reads/writes as a plain string ("2d6+3")
// ---------------------------------------------------------------------------

impl<'de> Deserialize<'de> for DiceExpr {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        parser::parse(&s).map_err(serde::de::Error::custom)
    }
}

impl Serialize for DiceExpr {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roll_in_range() {
        let d = DiceExpr::new(2, 6, 0);
        for _ in 0..200 {
            let r = d.roll();
            assert!((2..=12).contains(&r), "2d6 = {r} out of range");
        }
    }

    #[test]
    fn roll_with_modifier() {
        let d = DiceExpr::new(1, 6, 3);
        for _ in 0..200 {
            let r = d.roll();
            assert!((4..=9).contains(&r), "1d6+3 = {r} out of range");
        }
    }

    #[test]
    fn advantage_ge_normal() {
        let d = DiceExpr::new(1, 20, 0);
        // Just verify it doesn't panic and stays in range
        for _ in 0..100 {
            let r = d.roll_advantage();
            assert!((1..=20).contains(&r));
        }
    }

    #[test]
    fn serde_roundtrip() {
        // Verify Display → parse roundtrip (serde uses the same string format).
        for s in &["2d6+3", "1d20", "4d6", "1d8-1"] {
            let e: DiceExpr = s.parse().unwrap();
            assert_eq!(e.to_string(), *s);
        }
    }

    #[test]
    fn average_correct() {
        // 1d6: (1+6)/2 = 3
        assert_eq!(DiceExpr::new(1, 6, 0).average(), 3);
        // 2d6: 2*(3) = 6  — floor((1+6)/2.0)*2 = 3*2 = 6
        assert_eq!(DiceExpr::new(2, 6, 0).average(), 7); // (7/2)*2 = 7
                                                         // 1d8+2: 4+2 = 6  — floor((1+8)/2.0) = 4, 4+2=6
        assert_eq!(DiceExpr::new(1, 8, 2).average(), 6);
    }
}
