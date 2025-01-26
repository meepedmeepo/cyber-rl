use bracket_lib::prelude::Rect;

use super::{Map, TileType};

pub fn apply_room_to_map(map : &mut super::Map, room : &Rect) {
    for y in room.y1 +1 ..= room.y2 {
        for x in room.x1 + 1 ..= room.x2 {
            let idx = map.xy_idx(x, y);
            map.map[idx] = TileType::Floor;
        }
    }
}

pub fn apply_horizontal_tunnel(map : &mut super::Map, x1:i32, x2:i32, y:i32) -> Vec<usize>
{   
    let mut corridor = Vec::new();

    for x in std::cmp::min(x1,x2) ..= std::cmp::max(x1,x2) {
        let idx = map.xy_idx(x, y);
        if idx > 0 && idx < map.map_width as usize * map.map_height as usize {
            map.map[idx as usize] = TileType::Floor;

            corridor.push(idx as usize);
        }
    }

    corridor
}

pub fn apply_vertical_tunnel(map : &mut super::Map, y1:i32, y2:i32, x:i32) -> Vec<usize>
{
    let mut corridor = Vec::new();
    
    for y in std::cmp::min(y1,y2) ..= std::cmp::max(y1,y2) {
        let idx = map.xy_idx(x, y);
        if idx > 0 && idx < map.map_width as usize * map.map_height as usize {
            map.map[idx as usize] = TileType::Floor;
            corridor.push(idx as usize);
        }
    }

    corridor
}

pub fn draw_corridor(map: &mut Map, x1:i32, y1:i32, x2:i32, y2:i32) -> Vec<usize>
{
    let mut corridor = Vec::new();
    let mut x = x1;
    let mut y = y1;

    while x != x2 || y != y2 {
        if x < x2 {
            x += 1;
        } else if x > x2 {
            x -= 1;
        } else if y < y2 {
            y += 1;
        } else if y > y2 {
            y -= 1;
        }

        let idx = map.xy_idx(x, y);
        if map.map[idx] != TileType::Floor
        {
            map.map[idx] = TileType::Floor;
            corridor.push(idx);
        }
    }

    corridor
}
