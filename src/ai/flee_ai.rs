use bracket_lib::prelude::{console, DijkstraMap};
use hecs::Entity;

use crate::{
    map_indexing::SPATIAL_INDEX, FoV, HasMoved, Map, Player, Position, State, WantsToApproach,
    WantsToFlee,
};

use super::{apply_energy_cost, MyTurn};

pub fn flee_ai_system(state: &mut State) {
    let mut turn_done: Vec<Entity> = Vec::new();
    let mut has_moved: Vec<Entity> = Vec::new();

    for (ent, (pos, flee, fov, _turn)) in state
        .world
        .query_mut::<(&mut Position, &WantsToFlee, &mut FoV, &MyTurn)>()
        .without::<&Player>()
    {
        turn_done.push(ent);

        let my_idx = state.map.xy_idx(pos.x, pos.y);
        state.map.populate_blocked();

        let flee_map = bracket_lib::pathfinding::DijkstraMap::new(
            state.map.map_width as usize,
            state.map.map_height as usize,
            &flee.indices,
            &state.map,
            100.0,
        );

        let flee_target = DijkstraMap::find_highest_exit(&flee_map, my_idx, &state.map);

        if let Some(flee_target) = flee_target {
            let mut spatial_map = SPATIAL_INDEX.lock().unwrap();
            if !spatial_map.is_tile_blocked(flee_target as usize) {
                spatial_map.set_tile_unblocked_by_entity(my_idx);
                spatial_map.set_tile_blocked_by_entity(flee_target as usize);

                fov.dirty = true;
                pos.x = flee_target as i32 % state.map.map_width;
                pos.y = flee_target as i32 / state.map.map_width;
                has_moved.push(ent);
            }
        }
    }
    for moved in has_moved.iter() {
        let _ = state.world.remove_one::<HasMoved>(*moved);

        apply_energy_cost(state, super::ActionType::Move, *moved);
    }

    for done in turn_done.iter() {
        let _ = state.world.remove_one::<MyTurn>(*done);
        let _ = state.world.remove_one::<WantsToFlee>(*done);
    }
}
