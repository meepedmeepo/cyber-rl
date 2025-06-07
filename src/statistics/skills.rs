use std::collections::HashMap;

use hecs::Entity;

use crate::{
    gamelog::DEBUGLOG,
    statistics::{BaseStatistics, Pools, StatType},
    State,
};

pub struct Skills {
    skill_map: HashMap<Skill, i32>,
}

impl std::fmt::Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Skill::Hack => write!(f, "Hack"),
            Skill::Block => write!(f, "Block"),
            Skill::Dodge => write!(f, "Dodge"),
            Skill::Melee => write!(f, "Melee"),
            Skill::Ranged => write!(f, "Ranged"),
        }
    }
}

#[allow(dead_code)]
#[derive(Hash, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum Skill {
    Melee,
    Ranged,
    Dodge,
    Block,
    Hack,
}

pub fn get_skill_value(skill: Skill, entity: Entity, state: &mut State) -> i32 {
    let skills = state.world.query_one_mut::<&Skills>(entity).unwrap();

    *skills.skill_map.get(&skill).unwrap_or(&0)
}

///Rolls a skill check using a combination of skill level, stat modifier, and 1d20. Need to meet or beat the
///difficulty class for a success.
pub fn skill_check(
    skill: Skill,
    stat: StatType,
    entity: Entity,
    state: &mut State,
    difficulty_class: i32,
) -> bool {
    let mut query = state
        .world
        .query_one::<(&Skills, &Pools, &BaseStatistics)>(entity)
        .unwrap();

    let (skills, _pools, stats) = query.get().unwrap();

    let skill_mod = *skills.skill_map.get(&skill).unwrap_or(&0);
    let stat_mod = stats.get_stat(stat).get_modifier();
    let roll = state.rng.roll_dice(1, 20);
    let res = roll + skill_mod + stat_mod;

    let msg = format!(
        "Skill check: skill : {} ({}) + stat : {} ({}) + roll({}) = total ({})",
        skill.to_string(),
        skill_mod,
        stat.to_string(),
        stat_mod,
        roll,
        res
    );

    DEBUGLOG.add_log(msg.clone());
    state.game_log.add_log(msg);

    res >= difficulty_class
}
