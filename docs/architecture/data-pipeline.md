# Data Pipeline

## Flow

```
assets/<subsystem>/<file>.toml
        │
        ▼
src/data/types.rs        ← serde structs mirroring TOML schema
        │
        ▼
src/data/loader.rs       ← fn load<T: DeserializeOwned>(path) -> Result<T>
        │
        ▼
src/game/**              ← game modules consume typed structs
```

No game module reads raw TOML. All parsing is confined to `src/data/`.

---

## Key Design Rules

1. **One struct per asset type** in `types.rs` (e.g. `RegionManifest`,
   `QuestDef`, `DialogTree`, `MonsterStatBlock`).
2. `loader.rs` is the only place that calls `std::fs::read_to_string` and
   `toml::from_str`.
3. Assets are loaded eagerly at region load time, not on demand during combat,
   to avoid frame stutter.
4. Region assets are scoped: loading a new region calls
   `loader::load_region(slug)` which reads only `assets/regions/<slug>/`.
   The previous region's data is dropped.

---

## Region Loading in Detail

```rust
pub fn load_region(slug: &str) -> Result<LoadedRegion> {
    // 1. parse assets/regions/<slug>/region.toml  → RegionManifest
    // 2. for each room id in manifest: parse assets/regions/<slug>/rooms/<id>.toml
    // 3. for each npc id in manifest: parse assets/regions/<slug>/npcs/<id>.toml
    //    and assets/regions/<slug>/dialog/<id>.toml
    // 4. return LoadedRegion { manifest, rooms, npcs, dialogs }
}
```

This means an agent authoring a region only needs files inside that one folder.

See → [content/map-format.md](../content/map-format.md) for TOML schema details.

---

## Global Assets (loaded once at startup)

Loaded once and stored in `App`:
- All class definitions (`assets/classes/`)
- All race definitions (`assets/races/`)
- All monster stat blocks (`assets/monsters/`)
- All spell definitions (`assets/spells/`)
- All item definitions (`assets/items/`)
- All quest definitions (`assets/quests/`)
- All lore entries (`assets/lore/`)

These are small enough (< 2 MB total) that loading them all at startup is
acceptable and simplifies in-combat lookups.
