/// World representation — tile maps, rooms, regions, and field-of-view.
///
/// This module depends on `src/data/` (for loader types used in `Region`) but
/// has **no** dependency on any renderer or UI code.
///
/// ## Sub-modules
///
/// | Module      | Purpose                                             |
/// |-------------|-----------------------------------------------------|
/// | `map`       | `Tile` enum, `TileGrid` (2-D char grid, passability)|
/// | `room`      | `Room` — runtime room built from a `RoomDef` asset  |
/// | `region`    | `Region` — all rooms + manifest, built from loader  |
/// | `fov`       | Recursive shadowcasting FOV → `HashSet<(i32,i32)>`  |
///
/// ## Typical usage
///
/// ```rust,ignore
/// use crate::data::loader::load_region;
/// use crate::game::world::{Region, compute_fov};
///
/// let lr     = load_region("assets", "valley-of-ash")?;
/// let region = Region::from_loaded(&lr);
/// let room   = region.entry().unwrap();
/// let vis    = compute_fov((5, 3), 8, &room.grid);
/// ```
pub mod fov;
pub mod map;
pub mod region;
pub mod room;
pub mod weather;

// Convenience re-exports.
// pub use fov::compute as compute_fov;
// pub use map::{Tile, TileGrid};
// pub use region::Region;
// pub use room::Room;
