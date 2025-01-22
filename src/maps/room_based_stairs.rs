use bracket_lib::random::RandomNumberGenerator;

use super::{BuilderMap, Map, TileType};



pub struct RoomBasedStairs {}

impl RoomBasedStairs
{
    fn build(&mut self, _rng : &mut RandomNumberGenerator, build_data : &mut BuilderMap)
    {
        if let Some(rooms) = &build_data.rooms
        {
            let stair_pos = rooms[rooms.len()-1].center();
            
            let idx = Map::xy_id(stair_pos.x, stair_pos.y);

            build_data.map.map[idx] = TileType::DownStairs;
        }
        else 
        {
            panic!("Rooms existing is required for room based stair spawning!");    
        }
    }
}