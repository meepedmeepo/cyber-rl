use std::collections::HashMap;

use hecs::Entity;

use crate::{
    components::{GrantStat, StatusEffect},
    statistics::BaseStatistics,
    State,
};

pub fn stat_calculation_system(state: &mut State) {
    for (_, stats) in state.world.query_mut::<&mut BaseStatistics>() {
        stats.reset_stat_bonuses();
    }

    let stat_granting_effects = state
        .world
        .query_mut::<(&StatusEffect, &GrantStat)>()
        .into_iter()
        .map(|(_, (status, stat))| (*status, *stat))
        .collect::<Vec<_>>();

    for (status, stat) in stat_granting_effects.iter() {
        state
            .world
            .query_one_mut::<&mut BaseStatistics>(status.target)
            .unwrap()
            .change_stat_bonus(stat.stat, stat.amount);
    }
}
