use hecs::Entity;

use crate::{map_indexing::SPATIAL_INDEX, FoV, HasMoved, Map, Position, State, WantsToApproach};

use super::{apply_energy_cost, MyTurn};

pub fn approach_ai_system(state: &mut State) {
    let mut turn_done: Vec<Entity> = Vec::new();
    let mut has_moved: Vec<Entity> = Vec::new();

    for (ent, (_turn, approach, fov, pos)) in
        state
            .world
            .query_mut::<(&MyTurn, &WantsToApproach, &mut FoV, &mut Position)>()
    {
        turn_done.push(ent);

        let path = bracket_lib::pathfinding::a_star_search(
            state.map.xy_idx(pos.x, pos.y) as i32,
            state.map.xy_idx(
                approach.target % state.map.map_width,
                approach.target / state.map.map_width,
            ) as i32,
            &state.map,
        );

        if path.success && path.steps.len() > 1 {
            let mut spatial_map = SPATIAL_INDEX.lock().unwrap();

            let mut idx = state.map.xy_idx(pos.x, pos.y);
            spatial_map.set_tile_unblocked_by_entity(idx);

            pos.x = path.steps[1] as i32 % state.map.map_width;
            pos.y = path.steps[1] as i32 / state.map.map_width;

            has_moved.push(ent);

            idx = state.map.xy_idx(pos.x, pos.y);
            spatial_map.set_tile_blocked_by_entity(idx);

            fov.dirty = true;
        }
    }

    for moved in has_moved.iter() {
        let _ = state.world.insert_one(*moved, HasMoved {});

        apply_energy_cost(state, super::ActionType::Move, *moved);
    }

    for done in turn_done.iter() {
        let _ = state.world.remove_one::<WantsToApproach>(*done);
        let _ = state.world.remove_one::<MyTurn>(*done);
    }
}
