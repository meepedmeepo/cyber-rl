use std::collections::HashMap;

use hecs::Entity;

use crate::State;




pub struct Skills
{
    skill_map: HashMap<String, i32>,
}

#[allow(dead_code)]
pub enum Skill
{
    Melee,
    Ranged,
    Dodge,
    Block,
    Hack
}

pub fn get_skill_value(skill: String, entity: Entity, state: &mut State) -> i32
{
    let skills = state.world.query_one_mut::<&Skills>(entity).unwrap();

    *skills.skill_map.get(&skill).unwrap_or(&0)
}

