use bracket_lib::prelude::Point;
use hecs::Entity;

use crate::{map_indexing::SPATIAL_INDEX, statistics::Pools, State};

pub mod timer;

pub fn get_mob_entities_at_position(state: &mut State, pos: Point) -> Vec<Entity> {
    let mut hits = Vec::new();

    SPATIAL_INDEX.lock().unwrap().for_each_tile_content(
        state.map.xy_idx(pos.x, pos.y),
        state,
        |entity, state| {
            if let Ok(_) = state.world.get::<&Pools>(entity) {
                hits.push(entity);
            }
        },
    );

    hits
}

pub fn get_mobs_at_idx(state: &State, idx: usize) -> Vec<Entity> {
    SPATIAL_INDEX
        .lock()
        .unwrap()
        .get_tile_contents(idx)
        .iter()
        .filter(|ent| state.world.get::<&Pools>(**ent).is_ok())
        .cloned()
        .collect()
}
