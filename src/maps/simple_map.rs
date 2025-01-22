use bracket_lib::prelude::Point;
use bracket_lib::prelude::Rect;
use bracket_lib::random::RandomNumberGenerator;

use crate::spawns::spawning_system::spawn_room;
use crate::TileType;
use crate::common::*;
use crate::MAPHEIGHT;
use crate::MAPWIDTH;
use super::BuilderMap;
use super::InitialMapBuilder;
use super::MAPSIZE;
use super::{map, MapBuilder, Map};

pub struct SimpleMapBuilder
{
}

impl InitialMapBuilder for SimpleMapBuilder
{
    #[allow(dead_code)]
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data: &mut super::BuilderMap) {
        self.rooms_and_corridors(rng, build_data);
    }
}

impl SimpleMapBuilder {
    pub fn new(new_depth : i32) -> Box<SimpleMapBuilder>
    {
        Box::new(SimpleMapBuilder{})
    }
    fn rooms_and_corridors(&mut self,rng : &mut RandomNumberGenerator, build_data : &mut BuilderMap) 
    {
        const MAX_ROOMS : i32 = 30;
        const MIN_SIZE : i32 = 6;
        const MAX_SIZE : i32 = 10;

        let mut rooms : Vec<Rect> = Vec::new();


        for i in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, MAPWIDTH - w - 1) - 1;
            let y = rng.roll_dice(1, MAPHEIGHT - h - 1) - 1;
            let new_room = Rect::with_size(x, y, w, h);
            let mut ok = true;
            for other_room in rooms.iter() {
                if new_room.intersect(other_room) { ok = false }
            }
            if ok {
                apply_room_to_map(&mut build_data.map, &new_room);

                if !rooms.is_empty() {
                    let new_pos = new_room.center();
                    let new_x = new_pos.x;
                    let new_y = new_pos.y;

                    let prev_pos = rooms[rooms.len()-1].center();
                    let prev_x = prev_pos.x;
                    let prev_y = prev_pos.y;
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

                rooms.push(new_room);
            }
        }

        build_data.rooms = Some(rooms);
    }
}