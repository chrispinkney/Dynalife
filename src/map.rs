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

    // renders the map.
    // iterates over y and x values, matching the
    // values in the vec and assigning appropriately,
    // calling set to render each map tile.
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
}

// function to calculate a tile's index using tile's x and y values
// returns the index of a specified tile (in a vector)
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH + x) as usize)
}
