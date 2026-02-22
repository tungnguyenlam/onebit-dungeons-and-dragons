#[cfg(test)]
mod tests {
    use super::super::core::*;
    use super::super::report::*;

    #[test]
    fn validator_passes_repo_assets() {
        let report = validate_assets("assets").expect("validator should run");
        assert!(
            !report.has_errors(),
            "unexpected validation errors: {:?}",
            report.errors
        );
    }

    #[test]
    fn all_regions_have_multiple_rooms() {
        let regions_dir = std::path::Path::new("assets/regions");
        for entry in std::fs::read_dir(regions_dir).unwrap() {
            let path = entry.unwrap().path();
            if !path.is_dir() {
                continue;
            }
            let slug = path.file_name().unwrap().to_string_lossy().to_string();
            let loaded = crate::data::loader::load_region("assets", &slug).unwrap();
            assert!(
                loaded.manifest.rooms.len() >= 2,
                "region '{slug}' must have >=2 rooms, has {}",
                loaded.manifest.rooms.len()
            );
        }
    }

    #[test]
    fn all_rooms_reachable_from_entry() {
        use crate::data::types::TriggerKind;
        use std::collections::{HashSet, VecDeque};
        let regions_dir = std::path::Path::new("assets/regions");
        for entry in std::fs::read_dir(regions_dir).unwrap() {
            let path = entry.unwrap().path();
            if !path.is_dir() {
                continue;
            }
            let slug = path.file_name().unwrap().to_string_lossy().to_string();
            let loaded = crate::data::loader::load_region("assets", &slug).unwrap();
            let mut seen: HashSet<String> = HashSet::new();
            let mut q: VecDeque<String> = VecDeque::new();
            seen.insert(loaded.manifest.entry_room.clone());
            q.push_back(loaded.manifest.entry_room.clone());
            while let Some(rid) = q.pop_front() {
                if let Some(room) = loaded.rooms.get(&rid) {
                    for t in room
                        .triggers
                        .iter()
                        .filter(|t| t.kind == TriggerKind::Travel)
                    {
                        if loaded.rooms.contains_key(&t.target_id) {
                            if seen.insert(t.target_id.clone()) {
                                q.push_back(t.target_id.clone());
                            }
                        }
                    }
                }
            }
            for room_id in loaded.rooms.keys() {
                assert!(
                    seen.contains(room_id),
                    "[{slug}] room '{room_id}' unreachable"
                );
            }
        }
    }

    #[test]
    fn regions_have_branching_paths() {
        use crate::data::types::TriggerKind;
        let regions_dir = std::path::Path::new("assets/regions");
        for entry in std::fs::read_dir(regions_dir).unwrap() {
            let path = entry.unwrap().path();
            if !path.is_dir() {
                continue;
            }
            let slug = path.file_name().unwrap().to_string_lossy().to_string();
            let loaded = crate::data::loader::load_region("assets", &slug).unwrap();
            let branching = loaded.rooms.values().any(|room| {
                room.triggers
                    .iter()
                    .filter(|t| t.kind == TriggerKind::Travel)
                    .count()
                    >= 2
            });
            assert!(branching, "[{slug}] needs >=1 room with 2+ travel triggers");
        }
    }
}