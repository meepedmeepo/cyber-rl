use std::collections::{HashMap, HashSet};

use bracket_lib::{prelude::Rect, random::RandomNumberGenerator};

use super::{common::draw_corridor, BuilderMap, MetaMapBuilder};




pub struct CorridorsNearestNeighbour {}

impl MetaMapBuilder for CorridorsNearestNeighbour
{
    fn build_map(&mut self, rng: &mut bracket_lib::prelude::RandomNumberGenerator, build_data: &mut super::BuilderMap) 
    {
        self.corridors(rng, build_data);
    }
}

impl CorridorsNearestNeighbour
{
    pub fn new() -> Box<CorridorsNearestNeighbour>
    {
        Box::new(CorridorsNearestNeighbour {})
    }

    fn corridors(&mut self, _rng : &mut RandomNumberGenerator, build_data : &mut BuilderMap)
    {
        let rooms : Vec<Rect>;
        if let Some(room_builder) = &build_data.rooms
        {
            rooms = room_builder.clone();
        } else 
        {
            panic!("Corridors nearest neighbour requires a room based builder to have been ran first!");    
        }

        let mut connected : HashSet<usize> = HashSet::new();
        let mut corridors : Vec<Vec<usize>> = Vec::new();
        for(i, room) in rooms.iter().enumerate()
        {
            let mut room_distance : Vec<(usize, f32)> = Vec::new();
            let room_center = room.center();
            
            for (j, other_room) in rooms.iter().enumerate()
            {
                if i!= j && !connected.contains(&j)
                {
                    let other_center = other_room.center();
                    let distance = bracket_lib::pathfinding::DistanceAlg::Pythagoras.distance2d(room_center, other_center);

                    room_distance.push((j, distance));
                }
            }

            if !room_distance.is_empty()
            {
                room_distance.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());

                let dest_center = rooms[room_distance[0].0].center();
                let corridor = draw_corridor(&mut build_data.map, room_center.x, room_center.y, dest_center.x, dest_center.y);
                connected.insert(i);
                corridors.push(corridor);
            }
        }

        build_data.corridors = Some(corridors)
    }
}