use crate::app::App;
use std::fmt::Write;

impl App {
    /// Dump the current game state as a visual TUI-style text representation.
    /// This mimics the actual TUI layout with boxes and panels.
    pub fn dump_state(&self) -> String {
        use crate::game::world::map::Tile;

        let mut s = String::new();

        // Header - Region info
        let region_name = "Valley of Ash";
        let region_id = &self.current_room_id;
        writeln!(s, "┌World{:─<54}┐", "").unwrap();
        writeln!(s, "│Region: {} ({}) {:<35}│", region_name, region_id, "").unwrap();
        writeln!(s, "└{:─<66}┘", "").unwrap();

        // Status bar - Player info
        writeln!(s, "┌Status{:─<59}┐", "").unwrap();
        let hp_pct = self.player.current_hp as f32 / self.player.max_hp as f32;
        let hp_bar = "=".repeat((hp_pct * 10.0) as usize);
        writeln!(
            s,
            "│HP {} {}{}/{}│",
            self.player.name, hp_bar, self.player.current_hp, self.player.max_hp
        )
        .unwrap();
        writeln!(
            s,
            "│Gold: {}  XP: {}  Level: {}│",
            self.player.gold, self.player.xp, self.player.level
        )
        .unwrap();
        writeln!(s, "└{:─<66}┘", "").unwrap();

        // Map/Room display
        if let Some(room) = self.current_room() {
            writeln!(s, "┌Map{:─<63}┐", "").unwrap();

            let room_width = room.width() as usize;
            let room_height = room.height() as usize;

            // Top border
            writeln!(s, "│{:─<66}│", "").unwrap();

            // Room grid with player
            for y in 0..room_height {
                let mut line = String::from("│");
                for x in 0..room_width {
                    let tile = room.grid.get(x as u32, y as u32).unwrap_or(Tile::Wall);
                    let ch = match tile {
                        Tile::Floor => '.',
                        Tile::Wall => '#',
                        Tile::DoorClosed => '+',
                        Tile::DoorOpen => '/',
                        Tile::DeepWater => '~',
                        Tile::ShallowWater => '~',
                        Tile::StairsUp => '<',
                        Tile::StairsDown => '>',
                        Tile::Chest => '$',
                        Tile::NpcSpawn => '.',
                        Tile::Trigger => '!',
                        Tile::Unknown(_) => '?',
                    };
                    if (x as u32, y as u32) == self.player_pos {
                        line.push('@');
                    } else {
                        line.push(ch);
                    }
                }
                // Pad to width
                while line.len() < 67 {
                    line.push(' ');
                }
                line.push('│');
                writeln!(s, "{}", line).unwrap();
            }

            // Bottom border and padding
            for _ in 0..(20usize.saturating_sub(room_height)) {
                writeln!(s, "│{:─<66}│", "").unwrap();
            }
            writeln!(s, "└{:─<66}┘", "").unwrap();
        }

        // Controls bar
        writeln!(s, "┌Controls{:─<58}┐", "").unwrap();
        writeln!(
            s,
            "│Move: arrows/hjkl  Interact: Enter  ?: help             │"
        )
        .unwrap();
        writeln!(
            s,
            "│a combat  i inventory  s spellbook  n journal             │"
        )
        .unwrap();
        writeln!(s, "└{:─<66}┘", "").unwrap();

        // Feedback message if any
        if let Some(feedback) = self.get_feedback() {
            writeln!(s, "").unwrap();
            writeln!(s, "┌Message{:─<59}┐", "").unwrap();
            writeln!(s, "│{:<66}│", feedback).unwrap();
            writeln!(s, "└{:─<66}┘", "").unwrap();
        }

        s
    }
}
