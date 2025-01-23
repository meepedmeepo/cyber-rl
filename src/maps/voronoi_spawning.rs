use std::collections::HashMap;

use bracket_lib::{noise::{CellularDistanceFunction, FastNoise}, random::RandomNumberGenerator};

use crate::spawns::spawning_system::spawn_region;

use super::{BuilderMap, Map, MetaMapBuilder, TileType, MAPHEIGHT, MAPWIDTH};


pub struct VoronoiSpawning {}

impl MetaMapBuilder for VoronoiSpawning
{
    fn build_map(&mut self, rng: &mut bracket_lib::prelude::RandomNumberGenerator, build_data: &mut super::BuilderMap) 
    {
        self.build(rng, build_data);
    }
}

impl VoronoiSpawning
{
    #[allow(dead_code)]
    pub fn new() -> Box<VoronoiSpawning>
    {
        Box::new(VoronoiSpawning {})
    }

    fn build(&mut self, rng : &mut RandomNumberGenerator, build_data : &mut BuilderMap)
    {
        let mut noise_areas : HashMap<i32, Vec<usize>> = HashMap::new();
        let mut noise = FastNoise::seeded(rng.roll_dice(1, 65535) as u64);
        noise.set_noise_type(bracket_lib::noise::NoiseType::Cellular);
        noise.set_frequency(0.08);
        noise.set_cellular_distance_function(CellularDistanceFunction::Manhattan);

        for y in 1 .. MAPHEIGHT - 1
        {
            for x in 1 .. MAPWIDTH - 1
            {
                let idx = Map::xy_id(x, y);

                if build_data.map.map[idx] == TileType::Floor
                {
                    let cell_value_f = noise.get_noise(x as f32, y as f32) * 10240.0;
                    let cell_value = cell_value_f as i32;

                    if noise_areas.contains_key(&cell_value)
                    {
                        noise_areas.get_mut(&cell_value).unwrap().push(idx);
                    } else 
                    {
                        noise_areas.insert(cell_value, vec![idx]);    
                    }
                }
            }
        }
        //spawn entities
        for area in noise_areas.iter()
        {
            //todo add spawn region code when finished!
            spawn_region( &area.1, build_data.map.depth, &mut build_data.spawn_list);
        }
    }

}