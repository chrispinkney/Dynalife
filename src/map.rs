use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

// tiles can only be of type Wall or Floor
#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            // vector filled with NUM_TILES number of entries of type Floor
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    /// renders the map.
    /// iterates over y and x values, matching the
    /// values in the vec and assigning appropriately,
    /// calling set to render each map tile.
    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Floor => {
                        ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
                    }
                    TileType::Wall => {
                        ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
                    }
                }
            }
        }
    }

    /// Determines if the location specified in Point is grater than zero and thus is out of bounds.
    ///
    /// Returns: true if out of bounds.
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    /// Determines if the player can enter a specific tile. Checks if the destination is valid,
    /// then checks if that tile is of type Floor.
    ///
    /// Returns: true if player can enter.
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    /// Determines a given tile's index coordinates. Indicate an error of the tile's index is
    /// out of bounds.
    ///
    /// Returns: Some if valid, None if out of bounds.
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}

/// Calculates a tile's index using tile's x and y values
///
/// Returns: The index of a specified tile (in the map vector.)
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH + x) as usize)
}
