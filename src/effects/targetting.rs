use crate::{Map, Position};

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