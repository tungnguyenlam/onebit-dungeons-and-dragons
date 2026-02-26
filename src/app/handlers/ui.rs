use crate::app::state::{AppState, FocusedPane, JournalUiState};
use crate::app::App;
use crate::game::{
    items::equipment::EquipmentSlot,
    story::{
        dialog::{choose as dialog_choose, resolve as dialog_resolve},
        journal::Category as JournalCategory,
    },
};
use crate::renderer::{ControlFlow, GameEvent};
use anyhow::Result;

// Helper for rotating category
fn next_category(c: JournalCategory) -> JournalCategory {
    match c {
        JournalCategory::Quest => JournalCategory::Lore,
        JournalCategory::Lore => JournalCategory::World,
        JournalCategory::World => JournalCategory::Combat,
        JournalCategory::Combat => JournalCategory::Dialog,
        JournalCategory::Dialog => JournalCategory::System,
        JournalCategory::System => JournalCategory::Quest,
    }
}

fn prev_category(c: JournalCategory) -> JournalCategory {
    match c {
        JournalCategory::Quest => JournalCategory::System,
        JournalCategory::Lore => JournalCategory::Quest,
        JournalCategory::World => JournalCategory::Lore,
        JournalCategory::Combat => JournalCategory::World,
        JournalCategory::Dialog => JournalCategory::Combat,
        JournalCategory::System => JournalCategory::Dialog,
    }
}

impl App {
    pub fn handle_dialog(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::Cancel | GameEvent::Back => self.transition(AppState::WorldMap),
            GameEvent::Choice(n) => {
                let idx = n.saturating_sub(1) as usize;
                if let AppState::Dialog(ctx) = &mut self.state {
                    let Some(next) =
                        dialog_choose(&ctx.tree, &ctx.current_node, idx, &mut self.world_state)
                    else {
                        self.journal.append(
                            format!("dialog-blocked-{}-{}", ctx.tree.npc_id, self.turn),
                            self.turn,
                            JournalCategory::Dialog,
                            None,
                            format!("Talked with {}", ctx.npc_name),
                            "That option is unavailable right now.",
                        );
                        return Ok(());
                    };
                    if next == "END" {
                        self.transition(AppState::WorldMap);
                        return Ok(());
                    }
                    if let Some(resolved) = dialog_resolve(&ctx.tree, &next, &mut self.world_state)
                    {
                        ctx.current_node = next;
                        ctx.resolved = resolved;
                        self.journal.append(
                            format!("dialog-{}-{}", ctx.tree.npc_id, ctx.current_node),
                            self.turn,
                            JournalCategory::Dialog,
                            None,
                            format!("Talked with {}", ctx.npc_name),
                            ctx.resolved.text.clone(),
                        );
                    } else {
                        self.journal.append(
                            format!("dialog-broken-{}-{}", ctx.tree.npc_id, self.turn),
                            self.turn,
                            JournalCategory::Dialog,
                            None,
                            format!("Talked with {}", ctx.npc_name),
                            "Conversation path is blocked. Try another response or return later.",
                        );
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn handle_inventory(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::Back | GameEvent::Cancel => self.transition(AppState::WorldMap),
            GameEvent::Choice(1) => self.toggle_equip(EquipmentSlot::MainHand, "longsword"),
            GameEvent::Choice(2) => self.toggle_equip(EquipmentSlot::Armor, "leather_armor"),
            GameEvent::Choice(3) => self.toggle_equip(EquipmentSlot::OffHand, "shield"),
            GameEvent::Choice(4) => self.use_healing_potion(),
            _ => {}
        }
        Ok(())
    }

    pub fn handle_crafting(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::Back | GameEvent::Cancel => self.transition(AppState::WorldMap),
            GameEvent::Choice(n) if n >= 1 && n <= 9 => {
                let available = self.get_available_recipes();
                let idx = (n - 1) as usize;
                if let Some(recipe_id) = available.get(idx) {
                    self.craft_item(recipe_id);
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn handle_journal(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::Back | GameEvent::Cancel => {
                if self.focused_pane == FocusedPane::Side {
                    self.focused_pane = FocusedPane::Main;
                } else {
                    self.transition(AppState::WorldMap);
                }
            }
            GameEvent::Confirm => {
                if self.focused_pane == FocusedPane::Main {
                    self.focused_pane = FocusedPane::Side;
                }
            }
            GameEvent::MoveUp => {
                if self.focused_pane == FocusedPane::Main {
                    self.journal_ui.selected = self.journal_ui.selected.saturating_sub(1);
                    self.journal_ui.detail_scroll = 0;
                } else {
                    self.journal_ui.detail_scroll = self.journal_ui.detail_scroll.saturating_sub(1);
                }
            }
            GameEvent::MoveDown => {
                if self.focused_pane == FocusedPane::Main {
                    self.journal_ui.selected = self.journal_ui.selected.saturating_add(1);
                    self.journal_ui.detail_scroll = 0;
                } else {
                    self.journal_ui.detail_scroll = self.journal_ui.detail_scroll.saturating_add(1);
                }
            }
            GameEvent::MoveLeft => {
                if self.focused_pane == FocusedPane::Side {
                    self.focused_pane = FocusedPane::Main;
                } else {
                    self.journal_ui.category = prev_category(self.journal_ui.category);
                    self.journal_ui.selected = 0;
                    self.journal_ui.detail_scroll = 0;
                }
            }
            GameEvent::MoveRight => {
                if self.focused_pane == FocusedPane::Main {
                    self.journal_ui.category = next_category(self.journal_ui.category);
                    self.journal_ui.selected = 0;
                    self.journal_ui.detail_scroll = 0;
                }
            }
            GameEvent::OpenBestiary => self.transition(AppState::Bestiary),
            GameEvent::OpenLoreLibrary => self.transition(AppState::LoreLibrary),
            _ => {}
        }
        Ok(())
    }

    pub fn handle_bestiary(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::Back | GameEvent::Cancel => self.transition(AppState::Journal),
            _ => {}
        }
        Ok(())
    }

    pub fn handle_lore_library(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::Back | GameEvent::Cancel => self.transition(AppState::Journal),
            _ => {}
        }
        Ok(())
    }

    pub fn handle_ending(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::MoveDown => self.ending_scroll = self.ending_scroll.saturating_add(1),
            GameEvent::MoveUp => self.ending_scroll = self.ending_scroll.saturating_sub(1),
            GameEvent::Back | GameEvent::Cancel | GameEvent::Confirm => {
                self.ending_scroll = 0;
                self.transition(AppState::MainMenu);
            }
            _ => {}
        }
        Ok(())
    }

    pub fn handle_spellbook(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::Back | GameEvent::Cancel => self.transition(AppState::WorldMap),
            GameEvent::Choice(n @ 1..=9) => self.cast_known_spell((n - 1) as usize),
            _ => {}
        }
        Ok(())
    }
}
