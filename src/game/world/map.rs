/// Tile definitions and the 2-D tile grid.
///
/// Tiles are the atomic unit of the world. A `TileGrid` is parsed from the
/// `grid` field in a `room.toml` file and drives both rendering and
/// game-logic (passability, line-of-sight).
///
/// Tile legend (canonical — mirrors docs/gameplay/world.md):
///   `#`  Wall
///   `.`  Floor
///   `+`  Door (closed)
///   `-`  Door (open)
///   `~`  DeepWater
///   `,`  ShallowWater
///   `^`  Stairs up
///   `v`  Stairs down
///   `X`  Chest / interactable object
///   `@`  NPC spawn point (treated as floor at runtime)
///   `!`  Trigger zone (dialog / encounter / lore)
use std::fmt;

// ---------------------------------------------------------------------------
// Tile
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Wall,
    Floor,
    DoorClosed,
    DoorOpen,
    DeepWater,
    ShallowWater,
    StairsUp,
    StairsDown,
    Chest,
    NpcSpawn,
    Trigger,
    /// Any character not in the legend — treated as impassable for safety.
    Unknown(char),
}

impl Tile {
    /// Returns `true` if a creature can walk through this tile.
    pub fn is_passable(self) -> bool {
        matches!(
            self,
            Tile::Floor
                | Tile::DoorOpen
                | Tile::ShallowWater
                | Tile::StairsUp
                | Tile::StairsDown
                | Tile::Chest
                | Tile::NpcSpawn
                | Tile::Trigger
        )
    }

    /// Returns `true` if this tile blocks line-of-sight.
    pub fn blocks_sight(self) -> bool {
        matches!(self, Tile::Wall | Tile::DoorClosed)
    }

    /// The canonical display glyph for this tile.
    pub fn glyph(self) -> char {
        match self {
            Tile::Wall => '#',
            Tile::Floor => '.',
            Tile::DoorClosed => '+',
            Tile::DoorOpen => '-',
            Tile::DeepWater => '~',
            Tile::ShallowWater => ',',
            Tile::StairsUp => '^',
            Tile::StairsDown => 'v',
            Tile::Chest => 'X',
            Tile::NpcSpawn => '@',
            Tile::Trigger => '!',
            Tile::Unknown(c) => c,
        }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::Wall,
            '.' => Tile::Floor,
            '+' => Tile::DoorClosed,
            '-' => Tile::DoorOpen,
            '~' => Tile::DeepWater,
            ',' => Tile::ShallowWater,
            '^' => Tile::StairsUp,
            'v' => Tile::StairsDown,
            'X' => Tile::Chest,
            '@' => Tile::NpcSpawn,
            '!' => Tile::Trigger,
            c => Tile::Unknown(c),
        }
    }
}

// ---------------------------------------------------------------------------
// TileGrid
// ---------------------------------------------------------------------------

/// A 2-D grid of tiles (max 40 cols × 20 rows as per the room format spec).
///
/// Rows are stored in row-major order: `tiles[row][col]`.
#[derive(Debug, Clone)]
pub struct TileGrid {
    pub width: u32,
    pub height: u32,
    tiles: Vec<Vec<Tile>>,
}

impl TileGrid {
    /// Parse a multi-line string (the `grid` field from `room.toml`) into a
    /// `TileGrid`.
    ///
    /// Leading/trailing blank lines are stripped. All rows are right-padded
    /// with `Tile::Floor` to ensure a uniform width.
    pub fn from_str(s: &str) -> Self {
        let rows: Vec<Vec<Tile>> = s
            .lines()
            .map(str::trim_end)
            .filter(|l| !l.is_empty())
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();

        let width = rows.iter().map(Vec::len).max().unwrap_or(0) as u32;
        let height = rows.len() as u32;

        // Pad all rows to the same width.
        let tiles = rows
            .into_iter()
            .map(|mut row| {
                row.resize(width as usize, Tile::Floor);
                row
            })
            .collect();

        TileGrid {
            width,
            height,
            tiles,
        }
    }

    /// Get the tile at `(col, row)`, or `None` if the position is outside the
    /// grid bounds.
    pub fn get(&self, col: u32, row: u32) -> Option<Tile> {
        self.tiles.get(row as usize)?.get(col as usize).copied()
    }

    /// Returns `true` if `(col, row)` is within the grid and the tile is
    /// passable.
    pub fn is_passable(&self, col: i32, row: i32) -> bool {
        if col < 0 || row < 0 {
            return false;
        }
        self.get(col as u32, row as u32)
            .map(Tile::is_passable)
            .unwrap_or(false)
    }

    /// Returns `true` if `(col, row)` is outside the grid or the tile blocks
    /// line-of-sight.
    pub fn blocks_sight(&self, col: i32, row: i32) -> bool {
        if col < 0 || row < 0 {
            return true; // out-of-bounds is opaque
        }
        self.get(col as u32, row as u32)
            .map(Tile::blocks_sight)
            .unwrap_or(true) // out-of-bounds is opaque
    }

    /// Iterate over all `(col, row, tile)` triples.
    pub fn iter(&self) -> impl Iterator<Item = (u32, u32, Tile)> + '_ {
        self.tiles.iter().enumerate().flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .map(move |(c, &t)| (c as u32, r as u32, t))
        })
    }
}

impl fmt::Display for TileGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.tiles {
            for tile in row {
                write!(f, "{}", tile.glyph())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "
##########
#........#
#..@..!..#
#........#
##########
";

    #[test]
    fn parse_dimensions() {
        let g = TileGrid::from_str(SAMPLE);
        assert_eq!(g.width, 10);
        assert_eq!(g.height, 5);
    }

    #[test]
    fn wall_not_passable() {
        let g = TileGrid::from_str(SAMPLE);
        assert!(!g.is_passable(0, 0), "top-left corner is a wall");
    }

    #[test]
    fn floor_is_passable() {
        let g = TileGrid::from_str(SAMPLE);
        assert!(g.is_passable(1, 1), "interior floor tile");
    }

    #[test]
    fn wall_blocks_sight() {
        let g = TileGrid::from_str(SAMPLE);
        assert!(g.blocks_sight(0, 0), "wall blocks sight");
    }

    #[test]
    fn floor_does_not_block_sight() {
        let g = TileGrid::from_str(SAMPLE);
        assert!(!g.blocks_sight(1, 1), "floor does not block sight");
    }

    #[test]
    fn npc_spawn_is_passable() {
        let g = TileGrid::from_str(SAMPLE);
        // '@' is at col 3, row 2 in the SAMPLE grid
        assert!(g.is_passable(3, 2), "NPC spawn treated as passable");
    }

    #[test]
    fn trigger_is_passable() {
        let g = TileGrid::from_str(SAMPLE);
        // '!' is at col 6, row 2
        assert!(g.is_passable(6, 2), "trigger tile is passable");
    }

    #[test]
    fn out_of_bounds_impassable_and_opaque() {
        let g = TileGrid::from_str(SAMPLE);
        assert!(!g.is_passable(99, 99));
        assert!(g.blocks_sight(99, 99));
        assert!(!g.is_passable(-1, 0));
        assert!(g.blocks_sight(-1, 0));
    }

    #[test]
    fn display_roundtrip() {
        let s = "##\n#.\n";
        let g = TileGrid::from_str(s);
        assert_eq!(g.to_string(), "##\n#.\n");
    }
}
