use bracket_lib::{prelude::DijkstraMap, random::RandomNumberGenerator};

use super::{BuilderMap, Map, MetaMapBuilder, TileType, MAPHEIGHT, MAPWIDTH};



pub struct CullUnreachable {}

impl MetaMapBuilder for CullUnreachable
{
    fn build_map(&mut self, rng: &mut bracket_lib::prelude::RandomNumberGenerator, build_data: &mut super::BuilderMap) 
    {
        self.build(rng, build_data);
    }
}

impl CullUnreachable
{
    #[allow(dead_code)]
    pub fn new() -> Box<CullUnreachable>
    {
        Box::new(CullUnreachable {})
    }

    fn build(&mut self, _rng : &mut RandomNumberGenerator, build_data : &mut BuilderMap)
    {
        let starting_pos = build_data.starting_position.as_ref().unwrap().clone();
        let start_idx = Map::xy_id(starting_pos.x, starting_pos.y);

        build_data.map.populate_blocked();
        let map_starts : Vec<usize> = vec![start_idx];
        let dijkstra_map = DijkstraMap::new(MAPWIDTH, MAPHEIGHT, &map_starts, &build_data.map, 1000.0);

        for (i, tile) in build_data.map.map.iter_mut().enumerate()
        {
            if *tile == TileType::Floor
            {
                let distance_to_start = dijkstra_map.map[i];

                if distance_to_start == std::f32::MAX
                {
                    *tile = TileType::Wall;
                }
            }
        }


    }
}