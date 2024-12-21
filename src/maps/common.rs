use bracket_lib::prelude::Rect;

use super::{TileType, MAPHEIGHT, MAPWIDTH};

pub fn apply_room_to_map(map : &mut super::Map, room : &Rect) {
    for y in room.y1 +1 ..= room.y2 {
        for x in room.x1 + 1 ..= room.x2 {
            let idx = super::Map::xy_id(x, y);
            map.map[idx] = TileType::Floor;
        }
    }
}

pub fn apply_horizontal_tunnel(map : &mut super::Map, x1:i32, x2:i32, y:i32) {
    for x in std::cmp::min(x1,x2) ..= std::cmp::max(x1,x2) {
        let idx = super::Map::xy_id(x, y);
        if idx > 0 && idx < MAPWIDTH as usize * MAPHEIGHT as usize {
            map.map[idx as usize] = TileType::Floor;
        }
    }
}

pub fn apply_vertical_tunnel(map : &mut super::Map, y1:i32, y2:i32, x:i32) {
    for y in std::cmp::min(y1,y2) ..= std::cmp::max(y1,y2) {
        let idx = super::Map::xy_id(x, y);
        if idx > 0 && idx < MAPWIDTH as usize * MAPHEIGHT as usize {
            map.map[idx as usize] = TileType::Floor;
        }
    }
}