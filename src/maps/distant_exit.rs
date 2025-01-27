use bracket_lib::{prelude::DijkstraMap, random::RandomNumberGenerator};

use super::{tile_walkable, BuilderMap, Map, MetaMapBuilder, TileType};




pub struct DistantExitBuilder {}

impl MetaMapBuilder for DistantExitBuilder
{
    fn build_map(&mut self, rng: &mut bracket_lib::prelude::RandomNumberGenerator, build_data: &mut super::BuilderMap) 
    {
        self.build(rng, build_data);
    }
}


impl DistantExitBuilder
{
    pub fn new() -> Box<DistantExitBuilder>
    {
        Box::new(DistantExitBuilder {})
    }

    fn build(&mut self, _rng : &mut RandomNumberGenerator, build_data : &mut BuilderMap)
    {
        let starting_pos = build_data.starting_position.as_ref().unwrap().clone();
        let start_idx = build_data.map.xy_idx(starting_pos.x, starting_pos.y);

        build_data.map.populate_blocked();

        let mut exit_tile = (0, 0.0f32);
        let map_starts = vec![start_idx];
        let dijkstra_map = DijkstraMap::new(build_data.map.map_width as usize, build_data.map.map_height as usize, &map_starts
            , &build_data.map, 1000.0);

        for (i, tile) in build_data.map.map.iter_mut().enumerate()
        {
            if tile_walkable(*tile)
            {
                let distance_to_start = dijkstra_map.map[i];
                if distance_to_start != std::f32::MAX
                {
                    if distance_to_start > exit_tile.1
                    {
                        exit_tile.0 = i;
                        exit_tile.1 = distance_to_start;
                    }
                }
            }
        }

        let stairs_idx = exit_tile.0;
        build_data.map.map[stairs_idx] = TileType::DownStairs;
        
    }
}