use bracket_lib::pathfinding::DijkstraMap;

pub struct MapIndex {
    player_chase_map: DijkstraMapSearcher,
}

pub struct DijkstraMapSearcher {
    map: DijkstraMap,
    dirty: bool,
}

impl DijkstraMapSearcher {
    pub fn get_highest_exit(&self, tile_idx: usize) {
        //
    }
}
