use bracket_lib::prelude::Rect;
use bracket_lib::random::RandomNumberGenerator;

use crate::TileType;
use crate::common::*;
use crate::MAPHEIGHT;
use crate::MAPWIDTH;
use super::{map, MapBuilder, Map};

pub struct SimpleMapBuilder
{}

impl MapBuilder for SimpleMapBuilder
{
    fn build(new_depth: i32) -> Map 
    {
        let mut map = map::new(new_depth);
        SimpleMapBuilder::rooms_and_corridors(&mut map);
        map
    }
}

impl SimpleMapBuilder {
    fn rooms_and_corridors(map : &mut Map) {
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
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) { ok = false }
            }
            if ok {
                apply_room_to_map(map, &new_room);

                if !map.rooms.is_empty() {
                    let new_pos = new_room.center();
                    let new_x = new_pos.x;
                    let new_y = new_pos.y;
                    let prev_pos = map.rooms[map.rooms.len()-1].center();
                    let prev_x = prev_pos.x;
                    let prev_y = prev_pos.y;
                    if rng.range(0,2) == 1 {
                        apply_horizontal_tunnel(map, prev_x, new_x, prev_y);
                        apply_vertical_tunnel(map, prev_y, new_y, new_x);
                    } else {
                        apply_vertical_tunnel(map, prev_y, new_y, prev_x);
                        apply_horizontal_tunnel(map, prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        let stairs_position = map.rooms[map.rooms.len()-1].center();
        let stairs_idx = Map::xy_id(stairs_position.x, stairs_position.y);
        map.map[stairs_idx] = TileType::DownStairs;
    }
}