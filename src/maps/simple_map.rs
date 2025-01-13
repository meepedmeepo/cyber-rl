use bracket_lib::prelude::Point;
use bracket_lib::prelude::Rect;
use bracket_lib::random::RandomNumberGenerator;

use crate::spawns::spawning_system::spawn_room;
use crate::TileType;
use crate::common::*;
use crate::MAPHEIGHT;
use crate::MAPWIDTH;
use super::MAPSIZE;
use super::{map, MapBuilder, Map};

pub struct SimpleMapBuilder
{
    map : Map,
    starting_position : Point,
    depth : i32,
    rooms : Vec<Rect>,
}

impl MapBuilder for SimpleMapBuilder
{
    fn build(&mut self) -> Map 
    {
        self.map = map::new(self.depth);

        self.rooms_and_corridors();
        self.map.clone()
    }
    
    fn spawn_entities(&mut self, state : &mut crate::State) 
    {
        for room in self.rooms.iter()
        {
            spawn_room(state, *room, self.depth);
        }
    }
    
    fn get_map(&mut self) -> Map 
    {
        self.map.clone()
    }
    
    fn get_starting_position(&mut self) -> bracket_lib::prelude::Point 
    {
        self.starting_position
    }
}

impl SimpleMapBuilder {
    pub fn new(new_depth : i32) -> SimpleMapBuilder
    {
        SimpleMapBuilder{depth: new_depth, map : Map::new(vec![TileType::Wall;MAPSIZE]), starting_position : Point::zero(), rooms : Vec::new()}
    }
    fn rooms_and_corridors(&mut self) {
        const MAX_ROOMS : i32 = 30;
        const MIN_SIZE : i32 = 6;
        const MAX_SIZE : i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for i in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, MAPWIDTH - w - 1) - 1;
            let y = rng.roll_dice(1, MAPHEIGHT - h - 1) - 1;
            let new_room = Rect::with_size(x, y, w, h);
            let mut ok = true;
            for other_room in self.rooms.iter() {
                if new_room.intersect(other_room) { ok = false }
            }
            if ok {
                apply_room_to_map(&mut self.map, &new_room);

                if !self.rooms.is_empty() {
                    let new_pos = new_room.center();
                    let new_x = new_pos.x;
                    let new_y = new_pos.y;
                    let prev_pos = self.rooms[self.rooms.len()-1].center();
                    let prev_x = prev_pos.x;
                    let prev_y = prev_pos.y;
                    if rng.range(0,2) == 1 {
                        apply_horizontal_tunnel(&mut self.map, prev_x, new_x, prev_y);
                        apply_vertical_tunnel(&mut self.map, prev_y, new_y, new_x);
                    } else {
                        apply_vertical_tunnel(&mut self.map, prev_y, new_y, prev_x);
                        apply_horizontal_tunnel(&mut self.map, prev_x, new_x, new_y);
                    }
                }

                self.rooms.push(new_room);
            }
        }

        let stairs_position = self.rooms[self.rooms.len()-1].center();
        let stairs_idx = Map::xy_id(stairs_position.x, stairs_position.y);
        self.map.map[stairs_idx] = TileType::DownStairs;

        self.starting_position = self.rooms[0].center();
    }
}