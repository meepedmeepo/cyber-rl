use bracket_lib::prelude::{field_of_view, Point};

use crate::{FoV, Map, Position};

use super::{State, Entity};



pub fn entity_position(state: &mut State, target: Entity) -> Option<i32>
{
    match state.world.get::<&Position>(target)
    {
        Ok(pos) =>
        {
            return Some(Map::xy_id(pos.x, pos.y) as i32);
        }
        Err(_) => {None}
    }
}

pub fn get_aoe_tiles(state: &mut State, aoe : i32, pos : Point) -> Vec<i32>
{
    field_of_view(pos , aoe, &state.map)
        .iter()
        .map(|tile| Map::xy_id(tile.x, tile.y) as i32)
        .collect::<Vec<_>>()
        .clone()
}