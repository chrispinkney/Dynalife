use crate::prelude::*;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>, // helper for calculations involving rectangles
    pub player_start: Point,
}

impl MapBuilder {
    /// Iterate over all tiles in the tiles vector, changing all tiles into a wall.
    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    /// Generates a list of non-overlapping rooms.
    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        // generate rooms until we have NUM_ROOMS room
        while self.rooms.len() < NUM_ROOMS {
            // rectangular room with random length dimensions
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );

            // flag rooms as overlapped if they overlap
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }

            // if the rooms dont overlap, ensure they are within bounds and give them floors
            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });
                self.rooms.push(room)
            }
        }
    }

    /// Create a vertical tunnel between rooms
    fn apply_vertical_tunnel(&mut self, y1:i32, y2:i32, x:i32) {
        use std::cmp::{min, max};
        for y in min(y1,y2) ..=max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    /// Create a horizontal tunnel between rooms
    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y:i32) {
        use std::cmp::{min, max};
        for x in min(x1, x2) ..= max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    /// Builds the corridors between rooms
    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        // sort the rooms first by their center point to avoid giant snake-like tunnels connecting two rooms
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        // obtain center points of rooms where hallways will be connected
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i-1].center();
            let new = room.center();

            // diggy diggy hole
            if rng.range(0,2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }
}
