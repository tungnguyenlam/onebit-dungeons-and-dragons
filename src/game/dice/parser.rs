/// Dice expression parser.
///
/// Parses strings of the form `<count>d<sides>[+|-<modifier>]`.
/// Examples: `"1d20"`, `"2d6+3"`, `"1d8-1"`, `"4d6"`.
use super::DiceExpr;
use anyhow::{anyhow, Result};

/// Parse a dice expression from a string.
pub fn parse(s: &str) -> Result<DiceExpr> {
    let s = s.trim();

    // Find 'd' / 'D' delimiter
    let d_pos = s
        .find(|c| c == 'd' || c == 'D')
        .ok_or_else(|| anyhow!("invalid dice expression '{s}': missing 'd'"))?;

    // Die count (left of 'd') — defaults to 1 if omitted
    let count_str = &s[..d_pos];
    let count: u32 = if count_str.is_empty() {
        1
    } else {
        count_str
            .parse()
            .map_err(|_| anyhow!("invalid die count '{count_str}'"))?
    };

    let rest = &s[d_pos + 1..];

    // Modifier (right of the last '+' or '-')
    let (sides_str, modifier): (&str, i32) = if let Some(pos) = rest.rfind('+') {
        let m: i32 = rest[pos + 1..]
            .parse()
            .map_err(|_| anyhow!("invalid modifier in '{s}'"))?;
        (&rest[..pos], m)
    } else if let Some(pos) = rest.rfind('-') {
        let m: i32 = rest[pos + 1..]
            .parse()
            .map_err(|_| anyhow!("invalid modifier in '{s}'"))?;
        (&rest[..pos], -m)
    } else {
        (rest, 0)
    };

    let sides: u32 = sides_str
        .parse()
        .map_err(|_| anyhow!("invalid die sides '{sides_str}'"))?;

    if sides == 0 {
        return Err(anyhow!("die sides must be > 0 (got '{s}')"));
    }
    if count == 0 {
        return Err(anyhow!("die count must be > 0 (got '{s}')"));
    }

    Ok(DiceExpr { count, sides, modifier })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_basic() {
        let e = parse("2d6").unwrap();
        assert_eq!(e.count, 2);
        assert_eq!(e.sides, 6);
        assert_eq!(e.modifier, 0);
    }

    #[test]
    fn parses_with_positive_modifier() {
        let e = parse("1d20+5").unwrap();
        assert_eq!(e.count, 1);
        assert_eq!(e.sides, 20);
        assert_eq!(e.modifier, 5);
    }

    #[test]
    fn parses_with_negative_modifier() {
        let e = parse("1d8-1").unwrap();
        assert_eq!(e.modifier, -1);
    }

    #[test]
    fn parses_omitted_count() {
        let e = parse("d20").unwrap();
        assert_eq!(e.count, 1);
        assert_eq!(e.sides, 20);
    }

    #[test]
    fn roundtrip_display() {
        for s in &["1d20", "2d6+3", "4d6", "1d8-1"] {
            let e = parse(s).unwrap();
            assert_eq!(e.to_string(), s.to_string());
        }
    }

    #[test]
    fn rejects_bad_input() {
        assert!(parse("abc").is_err());
        assert!(parse("0d6").is_err());
        assert!(parse("1d0").is_err());
    }
}
