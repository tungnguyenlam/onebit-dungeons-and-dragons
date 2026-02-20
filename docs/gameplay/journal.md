# Journal

## Purpose

The journal is the player's in-game record of story events. It accumulates
entries automatically — the player never writes manually. Entries are grouped
by category for easy navigation.

---

## Entry Triggers

| Trigger type | Example |
|---|---|
| Quest stage advance | "You found tracks leading north…" |
| Dialog choice made | "You accepted the Captain's contract." |
| Lore item read | "An old journal entry about the Ash Wars." |
| World event fired | "The bandit army has mobilised." |
| Combat outcome | "The Bandit King has fallen." |

---

## JournalEntry Schema (`src/game/story/journal.rs`)

```rust
pub struct JournalEntry {
    pub id:        String,
    pub timestamp: u64,         // game turn number
    pub category:  Category,    // Quest | Lore | World | Combat
    pub quest_id:  Option<String>,
    pub title:     String,
    pub body:      String,
}

pub enum Category { Quest, Lore, World, Combat }
```

Entries are appended only, never edited or deleted. Stored in save file.

---

## Journal Entry Definitions

Entries are defined in quest TOML files (`journal_entry` field on each stage),
lore TOML files (`assets/lore/`), and emergent event definitions. The journal
system only stores the *resolved* text at the time of trigger — no templates
are re-evaluated later.

---

## Journal UI (`src/ui/screens/journal.rs`)

- Left panel: category tabs + list of entry titles, sorted newest-first
- Right panel: full body text of selected entry (word-wrapped, scrollable)
- Press `Tab` to cycle categories, arrow keys to navigate entries, `Esc` to close

New entries are marked with a `•` indicator in the HUD until the journal is
opened.
