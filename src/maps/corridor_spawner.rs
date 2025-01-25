use bracket_lib::random::RandomNumberGenerator;

use crate::spawns::spawning_system::spawn_region;

use super::{BuilderMap, MetaMapBuilder};





pub struct  CorridorSpawner {}

impl MetaMapBuilder for CorridorSpawner
{
    fn build_map(&mut self, rng: &mut bracket_lib::prelude::RandomNumberGenerator, build_data: &mut super::BuilderMap) 
    {
        self.build(rng, build_data);
    }
}

impl CorridorSpawner
{
    pub fn new() -> Box<CorridorSpawner>
    {
        Box::new(CorridorSpawner {})
    }

    fn build(&mut self, rng : &mut RandomNumberGenerator, build_data : &mut BuilderMap )
    {
        if let Some(corridors) = &build_data.corridors
        {
            for c in corridors.iter()
            {
                let depth = build_data.map.depth;

                spawn_region(&c, depth, &mut build_data.spawn_list);
            }
        } else 
        {
            panic!("Corridor spawner needs to run after builders that generate corridoor information first!");
        }
    }
}