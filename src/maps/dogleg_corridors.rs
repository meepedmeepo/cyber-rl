use bracket_lib::{prelude::Rect, random::RandomNumberGenerator};

use super::{common::{apply_horizontal_tunnel, apply_vertical_tunnel}, BuilderMap, MetaMapBuilder};




pub struct DoglegCorridors {}

impl MetaMapBuilder for DoglegCorridors
{
    fn build_map(&mut self, rng: &mut bracket_lib::prelude::RandomNumberGenerator, build_data: &mut super::BuilderMap) 
    {
        self.build(rng, build_data);
    }
}

impl DoglegCorridors
{
    fn new() -> Box<DoglegCorridors>
    {
        Box::new(DoglegCorridors{})
    }

    fn build(&mut self,  rng : &mut RandomNumberGenerator, build_data : &mut BuilderMap )
    {
        let rooms : Vec<Rect>;

        if let Some(rooms_builder) = &build_data.rooms
        {
            rooms = rooms_builder.clone();
        }else 
        {
            panic!("Dogleg corridors requires a builder that generates rooms to have been ran first!")    
        }

        for (i, room) in rooms.iter().enumerate()
        {
                if i > 0
                {
                    let (new_x, new_y) = room.center().to_tuple();

                    let (prev_x, prev_y) = rooms[rooms.len()-1].center().to_tuple();
                    if rng.range(0,2) == 1 
                    {
                        apply_horizontal_tunnel(&mut build_data.map, prev_x, new_x, prev_y);
                        apply_vertical_tunnel(&mut build_data.map, prev_y, new_y, new_x);
                    } else 
                    {
                        apply_vertical_tunnel(&mut build_data.map, prev_y, new_y, prev_x);
                        apply_horizontal_tunnel(&mut build_data.map, prev_x, new_x, new_y);
                    }
                }
        
        }
    }
}
