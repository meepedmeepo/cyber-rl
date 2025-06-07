use bracket_lib::prelude::SmallVec;

use crate::{
    map_indexing::spatial_index::SpatialIndexMap,
    maps::{map::Map, tile_cost},
};

impl SpatialIndexMap {
    fn get_available_exits(&self, idx: usize, base_map: &Map) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let x = idx as i32 % self.get_width() as i32;
        let y = idx as i32 / self.get_width() as i32;
        let w = self.get_width();
        let tt = base_map.map[idx];

        // Cardinal directions
        if self.is_exit_valid(x - 1, y) {
            exits.push((idx - 1, tile_cost(tt)))
        };
        if self.is_exit_valid(x + 1, y) {
            exits.push((idx + 1, tile_cost(tt)))
        };
        if self.is_exit_valid(x, y - 1) {
            exits.push((idx - w, tile_cost(tt)))
        };
        if self.is_exit_valid(x, y + 1) {
            exits.push((idx + w, tile_cost(tt)))
        };

        // Diagonals
        if self.is_exit_valid(x - 1, y - 1) {
            exits.push(((idx - w) - 1, tile_cost(tt) * 1.45));
        }
        if self.is_exit_valid(x + 1, y - 1) {
            exits.push(((idx - w) + 1, tile_cost(tt) * 1.45));
        }
        if self.is_exit_valid(x - 1, y + 1) {
            exits.push(((idx + w) - 1, tile_cost(tt) * 1.45));
        }
        if self.is_exit_valid(x + 1, y + 1) {
            exits.push(((idx + w) + 1, tile_cost(tt) * 1.45));
        }

        exits
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.get_width() as i32 - 1 || y < 1 || y > self.get_height() as i32 - 1 {
            return false;
        }

        //just a copy of xy_idx from Map -- TODO consider removing these functions from map and remove dependancy on
        // bracket lib completely
        let idx = (y as usize * self.get_width() as usize) + x as usize;

        !self.is_tile_blocked(idx)
    }
}
