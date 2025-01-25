use bracket_lib::{prelude::Rect, random::RandomNumberGenerator};

use super::{common::draw_corridor, BuilderMap, MetaMapBuilder};





pub struct BspCorridors {}

impl MetaMapBuilder for BspCorridors 
{
    fn build_map(&mut self, rng: &mut bracket_lib::prelude::RandomNumberGenerator, build_data: &mut super::BuilderMap) 
    {
        self.corridors(rng, build_data);
    }    
}


impl BspCorridors
{

    pub fn new() -> Box<BspCorridors>
    {
        Box::new(BspCorridors {})
    }

    fn corridors(&mut self, rng : &mut RandomNumberGenerator, build_data : &mut BuilderMap)
    {
        let rooms : Vec<Rect>;
        if let Some(room_builder) = &build_data.rooms
        {
            rooms = room_builder.clone();
        } else 
        {
            panic!("BSP corridors requires a room based builder has been ran first!");    
        }

        let mut corridors: Vec<Vec<usize>> = Vec::new();
        for i in 0..rooms.len()-1
        {
            let room = rooms[i];
            let next_room = rooms[i+1];
            let start_x = room.x1 + (rng.roll_dice(1, i32::abs(room.x1 - room.x2) -1));
            let start_y = room.y1 + (rng.roll_dice(1, i32::abs(room.y1 - room.y2) -1));
            let next_x = next_room.x1 + (rng.roll_dice(1, i32::abs(next_room.x1 - next_room.x2) -1));
            let next_y = next_room.y1 + (rng.roll_dice(1, i32::abs(next_room.y1 - next_room.y2) -1));

            let corridor = draw_corridor(&mut build_data.map, start_x, start_y, next_x, next_y);
            corridors.push(corridor);
        }

        build_data.corridors = Some(corridors);
    }
}