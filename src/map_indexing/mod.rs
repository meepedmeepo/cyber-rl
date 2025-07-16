mod pathing;
mod spatial_index;
use bracket_lib::{pathfinding::DijkstraMap, prelude::SmallVec};

pub use pathing::*;
pub use spatial_index::Movement;
pub use spatial_index::SPATIAL_INDEX;

pub struct MapIndex {
    player_chase_map: DijkstraMapSearcher,
}

pub struct DijkstraMapSearcher {
    map: DijkstraMap,
    dirty: bool,
}

impl DijkstraMapSearcher {
    pub fn get_lowest_exit(&self, tile_idx: usize) {
        //
        //bracket_lib::pathfinding::DijkstraMap::find_lowest_exit(dm, position, map)
    }
}

#[derive(Debug, Clone, Copy)]
///Struct used for list of blocked tiles in the spatial map
pub struct TileBlocked {
    blocked_by_map: bool,
    blocked_by_ent: bool,
}

impl TileBlocked {
    pub fn new(blocked_by_map: bool, blocked_by_ent: bool) -> Self {
        Self {
            blocked_by_map,
            blocked_by_ent,
        }
    }

    pub fn is_blocked(&self) -> bool {
        self.blocked_by_map || self.blocked_by_ent
    }

    pub fn set_map_block(&mut self, is_blocked: bool) {
        self.blocked_by_map = is_blocked
    }

    pub fn set_ent_block(&mut self, is_blocked: bool) {
        self.blocked_by_ent = is_blocked
    }
}

impl Default for TileBlocked {
    fn default() -> Self {
        Self {
            blocked_by_map: false,
            blocked_by_ent: false,
        }
    }
}
