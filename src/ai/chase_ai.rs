use std::collections::VecDeque;

use bracket_lib::prelude::{Point, a_star_search};
use hecs::Entity;

use crate::{
    Position, State,
    map_indexing::{Movement, SPATIAL_INDEX},
    statistics::BaseStatistics,
};

use super::MyTurn;

pub struct InCombat {
    pub target: Entity,
}

#[derive(Debug, Clone, Copy)]
pub struct Chasing {
    pub target: Entity,
    pub turns: u32,
}

pub struct LastKnownPosition {
    pub pos: Point,
}

#[derive(Debug, Clone)]
pub struct ChasePathCache {
    pub time_to_live: u32,
    pub path: VecDeque<usize>,
}

pub fn chase_ai_system(state: &mut State) {
    let mut turn_done = Vec::new();
    let mut chases_to_initiate = Vec::new();
    let mut chases_to_continue = Vec::new();

    for (entity, (_, combat, pos, stats, chase)) in state
        .world
        .query_mut::<(
            &MyTurn,
            &InCombat,
            &mut Position,
            &BaseStatistics,
            Option<&mut Chasing>,
        )>()
        .with::<&MyTurn>()
    {
        //

        if let Some(chasing) = chase {
            //continue chasing
            chasing.turns += 1;
            chases_to_continue.push((entity, *chasing, *pos));
        } else {
            //initiate a chase

            chases_to_initiate.push((entity, combat.target));
        }
    }

    for (ent, target) in chases_to_initiate.iter() {
        let _ = state.world.insert_one(
            *ent,
            Chasing {
                target: *target,
                turns: 0,
            },
        );
    }

    let mut moves_to_make: Vec<(Entity, Movement)> = Vec::new();
    let mut cache_to_remove: Vec<Entity> = Vec::new();
    let mut cache_to_create: Vec<(Entity, ChasePathCache)> = Vec::new();

    for (ent, chase, pos) in chases_to_continue.iter() {
        //
        let res = crate::statistics::stat_check(
            crate::statistics::StatType::Intelligence,
            *ent,
            state,
            0 + chase.turns as i32,
        );

        match res {
            false => {
                //stop chase

                state
                    .world
                    .remove_one::<Chasing>(*ent)
                    .expect("Couldn't remove chase from entity.");

                let _ = state.world.remove_one::<InCombat>(*ent);
            }

            true => {
                //continue chase

                let cache = state.world.get::<&mut ChasePathCache>(*ent);

                if let Ok(mut cache) = cache
                    && !cache.path.is_empty()
                {
                    //attempt to follow path

                    let front = *cache.path.front().unwrap();
                    let old_pos = state.map.xy_idx(pos.x, pos.y);

                    if !SPATIAL_INDEX.lock().unwrap().is_tile_blocked(front) {
                        //path not blocked so move
                        moves_to_make.push((
                            *ent,
                            Movement {
                                old_pos,
                                new_pos: front,
                            },
                        ));

                        cache.path.pop_front();
                    } else {
                        //can't make move so wait a turn and decrease cache time_to_live
                        cache.time_to_live -= 1;
                        if cache.time_to_live < 1 {
                            cache_to_remove.push(*ent);
                        } else {
                            turn_done.push(*ent);
                        }
                    }
                } else {
                    //no cache exists so create one with A* path
                    let ent_pos = state.world.get::<&Position>(*ent).unwrap().as_tuple();
                    let start_idx = state.map.xy_idx(ent_pos.0, ent_pos.1);

                    let target_pos = state
                        .world
                        .get::<&Position>(chase.target)
                        .unwrap()
                        .as_tuple();
                    let end_idx = state.map.xy_idx(target_pos.0, target_pos.1);

                    let cache = a_star_search(start_idx, end_idx, &state.map)
                        .steps
                        .iter()
                        .skip(1)
                        .cloned()
                        .collect::<VecDeque<usize>>();

                    let cache_comp = ChasePathCache {
                        time_to_live: 2,
                        path: cache,
                    };

                    cache_to_create.push((*ent, cache_comp));
                }
            }
        }
    }

    for (ent, cache) in cache_to_create.into_iter() {
        state
            .world
            .insert_one(ent, cache)
            .expect("Couldn't insert new ChasePathCache Component");
    }

    for ent in cache_to_remove.into_iter() {
        let _ = state.world.remove_one::<ChasePathCache>(ent);
    }

    //Make all entity movements, and end their turn
    for (ent, movement) in moves_to_make.into_iter() {
        SPATIAL_INDEX
            .lock()
            .unwrap()
            .move_entity(ent, movement, state);

        turn_done.push(ent);
    }

    //ends turn for all enemies that made a movement to chase
    for ent in turn_done.iter() {
        let _ = state.world.remove_one::<MyTurn>(*ent);
    }
}
