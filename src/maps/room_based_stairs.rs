use bracket_lib::random::RandomNumberGenerator;

use super::{BuilderMap, Map, MetaMapBuilder, TileType};



pub struct RoomBasedStairs {}

impl MetaMapBuilder for RoomBasedStairs
{
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data: &mut BuilderMap) {
        self.build(rng, build_data);
    }
}

impl RoomBasedStairs
{
    pub fn new() -> Box<RoomBasedStairs>
    {
        Box::new(RoomBasedStairs {})
    }
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