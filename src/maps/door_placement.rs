use bracket_lib::random::RandomNumberGenerator;

use super::{BuilderMap, MetaMapBuilder, TileType, MAPHEIGHT, MAPWIDTH};




pub struct DoorPlacement {}

impl MetaMapBuilder for DoorPlacement
{
    fn build_map(&mut self, rng: &mut bracket_lib::prelude::RandomNumberGenerator, build_data: &mut super::BuilderMap) 
    {
        self.doors(rng, build_data);
    }
}

impl DoorPlacement
{
    pub fn new() -> Box<DoorPlacement>
    {
        Box::new(DoorPlacement {})
    }

    fn doors(&mut self, _rng : &mut RandomNumberGenerator, build_data : &mut BuilderMap)
    {
        if let Some(halls_original) = &build_data.corridors
        {
            let halls = halls_original.clone();

            for hall in halls.iter()
            {
                if hall.len() > 2
                {
                    if self.door_possible(build_data, hall[0])
                    {
                        build_data.spawn_list.push((hall[0], "Door".to_string()));
                    }
                }
            }
        } else 
        {
            let tiles = build_data.map.map.clone();
            for (i, tile) in tiles.iter().enumerate()
            {
                if *tile == TileType::Floor && self.door_possible(build_data, i)
                {
                    build_data.spawn_list.push((i, "Door".to_string()));
                }
            }
        }
    }

    fn door_possible(&self, build_data : &mut BuilderMap, idx : usize) -> bool
    {
        let x = idx % MAPWIDTH as usize;
        let y = idx / MAPWIDTH as usize;

        //check for east->west door possibility
        if build_data.map.map[idx] == TileType::Floor && (x > 1 && build_data.map.map[idx-1] == TileType::Floor) &&
            (x < MAPWIDTH as usize -2 && build_data.map.map[idx+1] == TileType::Floor) &&
            (y > 1 && build_data.map.map[idx - MAPWIDTH as usize] == TileType::Wall) &&
            (y < MAPHEIGHT as usize-2 && build_data.map.map[idx + MAPWIDTH as usize] == TileType::Wall)
        {
            return true;
        }

        // Check for north-south door possibility
        if build_data.map.map[idx] == TileType::Floor &&
            (x > 1 && build_data.map.map[idx-1] == TileType::Wall) &&
            (x < MAPWIDTH as usize -2 && build_data.map.map[idx+1] == TileType::Wall) &&
            (y > 1 && build_data.map.map[idx - MAPWIDTH as usize] == TileType::Floor) &&
            (y < MAPHEIGHT as usize-2 && build_data.map.map[idx + MAPWIDTH as usize] == TileType::Floor)
        {
            return true;
        }   


        false
    }
}