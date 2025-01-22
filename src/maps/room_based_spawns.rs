use bracket_lib::random::RandomNumberGenerator;

use crate::spawns::spawning_system::spawn_room;

use super::{BuilderMap, MetaMapBuilder};




pub struct RoomBasedSpawns {}

impl MetaMapBuilder for RoomBasedSpawns  
{
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data: &mut BuilderMap) 
    {
        self.build(rng, build_data);
    }
}

impl RoomBasedSpawns
{
    fn new() -> Box<RoomBasedSpawns>
    {
        Box::new(RoomBasedSpawns{})
    }

    fn build(&mut self, rng : &mut RandomNumberGenerator, build_data : &mut BuilderMap)
    {
        if let Some(rooms) = &build_data.rooms
        {
            for room in rooms.iter()
            {
                spawn_room( *room, build_data.map.depth, &mut build_data.spawn_list, &build_data.map);
            }
        }
        else 
        {
            panic!("Room based spawning requires rooms to have been made in order to spawn!");    
        }
    }
}