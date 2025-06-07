use hecs::Entity;

use crate::{
    components::{BlocksTiles, BlocksVisibility, FoV, Renderable},
    visibility_system::VisibilitySystem,
    State,
};

pub fn open_door(state: &mut State, interactor: Entity, door: Entity) {
    let _ = state.world.remove_one::<BlocksTiles>(door);
    let _ = state.world.remove_one::<BlocksVisibility>(door);
    state
        .world
        .query_one_mut::<&mut Renderable>(door)
        .unwrap()
        .glyph = "/".to_string();
    state
        .world
        .query_one_mut::<&mut FoV>(state.player_ent.unwrap())
        .unwrap()
        .dirty = true;

    VisibilitySystem::run(state);
}

pub fn close_door(state: &mut State, interactor: Entity, door: Entity) {
    let _ = state.world.insert_one(door, BlocksTiles {});
    let _ = state.world.insert_one(door, BlocksVisibility {});
    state
        .world
        .query_one_mut::<&mut Renderable>(door)
        .unwrap()
        .glyph = "+".to_string();
    state
        .world
        .query_one_mut::<&mut FoV>(state.player_ent.unwrap())
        .unwrap()
        .dirty = true;

    VisibilitySystem::run(state);
}
