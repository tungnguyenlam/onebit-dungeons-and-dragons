use crate::data::loader::load_region;

#[test]
fn test_tidewatch_coast_loading() {
    println!("Testing Tidewatch Coast region loading...");

    match load_region("assets", "tidewatch-coast") {
        Ok(loaded_region) => {
            println!("✅ Region loaded successfully!");
            assert_eq!(loaded_region.manifest.name, "Tidewatch Coast");
            assert_eq!(loaded_region.manifest.entry_room, "dockside");
            assert_eq!(loaded_region.rooms.len(), 4);
            assert_eq!(loaded_region.npcs.len(), 2);
            assert!(loaded_region.rooms.contains_key("dockside"));
            assert!(loaded_region.rooms.contains_key("market-square"));
            assert!(loaded_region.rooms.contains_key("harbormaster"));
            assert!(loaded_region.rooms.contains_key("sea-cave"));
            assert!(loaded_region.npcs.contains_key("merchant_lyr"));
            assert!(loaded_region.npcs.contains_key("harbormaster"));
        }
        Err(e) => {
            panic!("Failed to load Tidewatch Coast region: {}", e);
        }
    }
}
