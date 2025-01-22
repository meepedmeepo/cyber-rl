use bracket_lib::{prelude::Point, random::RandomNumberGenerator};

use super::{BuilderMap, MetaMapBuilder};



pub struct RoomBasedStartingPosition {}

impl MetaMapBuilder for RoomBasedStartingPosition
{
    fn build_map(&mut self, rng: &mut bracket_lib::prelude::RandomNumberGenerator, build_data: &mut super::BuilderMap) {
        self.build(rng, build_data);
    }
}

impl RoomBasedStartingPosition
{
    fn new() -> Box<RoomBasedStartingPosition>
    {
        Box::new(RoomBasedStartingPosition{})
    }

    fn build(&mut self, rng : &mut RandomNumberGenerator, build_data : &mut BuilderMap)
    {
        match &build_data.rooms
        {
            None => panic!("Rooms need to exist to choose a room based starting position!"),
            Some(rooms) => 
            {
                let start_pos = rooms[0].center();
                build_data.starting_position = Some(Point{x: start_pos.x, y: start_pos.y});
            }
        }
    }
}