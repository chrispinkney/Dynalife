use crate::prelude::*;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>, // helper for calculations involving rectangles
    pub player_start: Point,
}

// iterate over all tiles in the tiles vector, changing all tiles into a wall
fn fill(&mut self, tile : TileType) {
    self.map.tiles.iter_mut().for_each(|t| *t = tile);
}
