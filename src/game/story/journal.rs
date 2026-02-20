/// Journal storage for story/world/combat entries.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    Quest,
    Lore,
    World,
    Combat,
    Dialog,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JournalEntry {
    pub id:        String,
    pub timestamp: u64,
    pub category:  Category,
    pub quest_id:  Option<String>,
    pub title:     String,
    pub body:      String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Journal {
    pub entries: Vec<JournalEntry>,
    pub has_unread: bool,
}

impl Journal {
    pub fn append(
        &mut self,
        id: impl Into<String>,
        timestamp: u64,
        category: Category,
        quest_id: Option<String>,
        title: impl Into<String>,
        body: impl Into<String>,
    ) {
        self.entries.push(JournalEntry {
            id: id.into(),
            timestamp,
            category,
            quest_id,
            title: title.into(),
            body: body.into(),
        });
        self.has_unread = true;
    }

    pub fn mark_read(&mut self) {
        self.has_unread = false;
    }

    pub fn entries_by_category(&self, category: Category) -> Vec<&JournalEntry> {
        let mut entries: Vec<&JournalEntry> = self
            .entries
            .iter()
            .filter(|e| e.category == category)
            .collect();
        entries.sort_by_key(|e| std::cmp::Reverse(e.timestamp));
        entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_sets_unread() {
        let mut j = Journal::default();
        j.append("e1", 1, Category::Quest, None, "t", "b");
        assert!(j.has_unread);
        assert_eq!(j.entries.len(), 1);
    }

    #[test]
    fn category_filter_is_newest_first() {
        let mut j = Journal::default();
        j.append("old", 1, Category::Quest, None, "old", "old");
        j.append("new", 9, Category::Quest, None, "new", "new");
        j.append("lore", 10, Category::Lore, None, "l", "l");
        let quest = j.entries_by_category(Category::Quest);
        assert_eq!(quest.len(), 2);
        assert_eq!(quest[0].id, "new");
        assert_eq!(quest[1].id, "old");
    }
}
