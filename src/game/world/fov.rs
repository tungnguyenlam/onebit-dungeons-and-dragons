/// Field-of-view computation using recursive shadowcasting.
///
/// Algorithm: Björn Bergström's recursive shadowcasting
/// (http://www.roguebasin.com/index.php/FOV_using_recursive_shadowcasting)
///
/// The world is divided into 8 octants. For each octant the algorithm scans
/// increasing-distance rows and maintains a visible slope window [`start`,
/// `end`]. Opaque tiles narrow the window; transparent tiles are lit.
///
/// `compute(origin, radius, grid)` returns the `HashSet` of `(col, row)`
/// positions visible from `origin` within `radius` tiles.
use crate::game::world::map::TileGrid;
use std::collections::HashSet;

// ---------------------------------------------------------------------------
// Octant transformation multipliers
// Rows: [xx, xy, yx, yy] for each of the 8 octants.
// ---------------------------------------------------------------------------

const MULT: [[i32; 8]; 4] = [
    [ 1,  0,  0, -1, -1,  0,  0,  1], // xx
    [ 0,  1, -1,  0,  0, -1,  1,  0], // xy
    [ 0,  1,  1,  0,  0, -1, -1,  0], // yx
    [ 1,  0,  0,  1, -1,  0,  0, -1], // yy
];

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Compute the set of `(col, row)` positions visible from `origin` within
/// `radius` tiles on `grid`.
///
/// The origin tile is always included. Tiles whose centre is beyond `radius`
/// (Euclidean) are excluded. Opaque tiles (walls, closed doors) are included
/// in the set if they are at the boundary of the visible arc, so the player
/// can see *that* a wall is there.
pub fn compute(origin: (i32, i32), radius: u32, grid: &TileGrid) -> HashSet<(i32, i32)> {
    let mut visible = HashSet::new();
    visible.insert(origin);

    if radius == 0 {
        return visible;
    }

    let r = radius as i32;
    let (cx, cy) = origin;

    for octant in 0..8usize {
        cast_light(
            &mut visible,
            grid,
            cx,
            cy,
            1,     // starting row (distance step)
            1.0,   // start slope
            0.0,   // end slope
            r,
            MULT[0][octant],
            MULT[1][octant],
            MULT[2][octant],
            MULT[3][octant],
        );
    }

    visible
}

// ---------------------------------------------------------------------------
// Internal shadowcasting
// ---------------------------------------------------------------------------

/// Recursive light-casting function for one octant.
///
/// # Parameters
/// - `row`   — distance step (1 = adjacent; increases recursively)
/// - `start` — left-edge slope of the current visible arc (begins at 1.0)
/// - `end`   — right-edge slope of the current visible arc (begins at 0.0)
/// - `xx`,`xy`,`yx`,`yy` — octant rotation coefficients from `MULT`
#[allow(clippy::too_many_arguments)]
fn cast_light(
    visible: &mut HashSet<(i32, i32)>,
    grid:    &TileGrid,
    cx:      i32,
    cy:      i32,
    row:     i32,
    start:   f32,
    end:     f32,
    radius:  i32,
    xx:      i32,
    xy:      i32,
    yx:      i32,
    yy:      i32,
) {
    if start < end {
        return;
    }
    let radius_sq = (radius * radius) as f32;

    let mut start = start;
    let mut j = row;

    'row: while j <= radius {
        let dy = -j;
        // dx starts at -(j+1); the first thing inside the loop increments it
        // to -j, so the scan range per row is -j ..= 0.
        let mut dx      = -(j + 1);
        let mut blocked = false;
        let mut new_start = 0.0f32; // set before read, init is defensive only

        while dx <= 0 {
            dx += 1;

            // Map octant coordinates to world coordinates.
            let x = cx + dx * xx + dy * xy;
            let y = cy + dx * yx + dy * yy;

            // Slope of the left and right edges of this tile's cell.
            let l_slope = (dx as f32 - 0.5) / (dy as f32 + 0.5);
            let r_slope = (dx as f32 + 0.5) / (dy as f32 - 0.5);

            // Skip tiles outside the current arc.
            if start < r_slope {
                continue;
            }
            if end > l_slope {
                break;
            }

            // Tile is within the arc — light it if within the Euclidean radius.
            let dist_sq = (dx * dx + dy * dy) as f32;
            if dist_sq < radius_sq {
                visible.insert((x, y));
            }

            if blocked {
                // Currently scanning a run of blocked tiles.
                if grid.blocks_sight(x, y) {
                    new_start = r_slope;
                } else {
                    // Transition: blocked → open. Resume arc from new_start.
                    blocked = false;
                    start   = new_start;
                }
            } else if grid.blocks_sight(x, y) && j < radius {
                // Transition: open → blocked. Recurse with narrowed arc then
                // record where the next potential open run begins.
                blocked = true;
                cast_light(visible, grid, cx, cy, j + 1, start, l_slope,
                           radius, xx, xy, yx, yy);
                new_start = r_slope;
            }
        }

        // If the last tile of this row was blocked, all further rows in this
        // octant scan are fully occluded.
        if blocked {
            break 'row;
        }
        j += 1;
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_always_visible() {
        let grid = TileGrid::from_str(".....\n.....\n.....\n.....\n.....\n");
        let vis  = compute((2, 2), 5, &grid);
        assert!(vis.contains(&(2, 2)), "origin must be visible");
    }

    #[test]
    fn zero_radius_only_origin() {
        let grid = TileGrid::from_str(".....\n.....\n.....\n");
        let vis  = compute((2, 1), 0, &grid);
        assert_eq!(vis.len(), 1);
        assert!(vis.contains(&(2, 1)));
    }

    #[test]
    fn open_room_fully_visible_from_centre() {
        // 5×5 room with surrounding walls; player stands at centre (2,2).
        let grid = TileGrid::from_str(
            "#####\n\
             #...#\n\
             #...#\n\
             #...#\n\
             #####\n",
        );
        let vis = compute((2, 2), 10, &grid);
        // All 9 interior floor tiles should be visible.
        for row in 1..=3 {
            for col in 1..=3 {
                assert!(vis.contains(&(col, row)), "({col},{row}) should be visible");
            }
        }
    }

    #[test]
    fn wall_blocks_tiles_behind_it() {
        // Double-wall row separating two chambers.
        //   row 0: #####
        //   row 1: #...#  ← player here at (2,1)
        //   row 2: #####  ← solid wall
        //   row 3: #...#  ← hidden chamber
        //   row 4: #####
        let grid = TileGrid::from_str(
            "#####\n\
             #...#\n\
             #####\n\
             #...#\n\
             #####\n",
        );
        let vis = compute((2, 1), 10, &grid);
        // Row 3 is completely behind the wall and should not be visible.
        for col in 0..5 {
            assert!(!vis.contains(&(col, 3)), "({col},3) should be hidden");
        }
    }

    #[test]
    fn radius_limits_visibility() {
        let grid = TileGrid::from_str(
            "...........\n\
             ...........\n\
             ...........\n",
        );
        let vis = compute((5, 1), 2, &grid);
        // (0,1) is 5 tiles away and must be invisible.
        assert!(!vis.contains(&(0, 1)), "tile beyond radius must be hidden");
    }

    #[test]
    fn pillar_casts_shadow() {
        // A single wall tile at (3,1) should shadow tiles directly behind it.
        //  col: 0123456
        //  row 0: .......
        //  row 1: ...#...   ← pillar at (3,1)
        //  row 2: .......
        //  row 3: .......
        let grid = TileGrid::from_str(
            ".......\n\
             ...#...\n\
             .......\n\
             .......\n",
        );
        // Player is directly west of the pillar at (1,1).
        let vis = compute((1, 1), 10, &grid);
        // (5,1) is directly behind the pillar from (1,1) and should be hidden.
        assert!(!vis.contains(&(5, 1)), "(5,1) should be in the pillar's shadow");
    }
}
