use crate::{
    components::{FoV, Hidden, Name},
    gamelog::DEBUGLOG,
    map_indexing::SPATIAL_INDEX,
    statistics::BaseStatistics,
    State,
};

///ran each turn when initiative is calculated - currently only the player is able to reveal traps and they will be revealed for all entities
pub fn spot_traps(state: &mut State) {
    let query = state
        .world
        .query_one_mut::<&FoV>(state.player_ent.unwrap())
        .unwrap();
    let fov = query.clone();

    for tile in fov.visible_tiles.iter() {
        let idx = state.map.xy_idx(tile.x, tile.y) as i32;

        match SPATIAL_INDEX.lock().unwrap().get_prop_entity_at_idx(idx) {
            Some(prop) => {
                if state.world.get::<&Hidden>(prop).is_ok() {
                    let roll = state.rng.roll_dice(1, 20);
                    let bonus = state
                        .world
                        .query_one_mut::<&BaseStatistics>(state.player_ent.unwrap())
                        .unwrap()
                        .intelligence
                        .get_modifier();
                    DEBUGLOG.add_log(format!(
                        "Spot Trap: Roll: {} Bonus: {} Total: {}",
                        roll,
                        bonus,
                        roll + bonus
                    ));
                    if roll + bonus >= 21 || roll == 20 {
                        let _ = state.world.remove_one::<Hidden>(prop);
                        let name = state.world.get::<&Name>(prop).unwrap().name.clone();
                        state.game_log.add_log(format!("{} spotted!", name));
                    }
                }
            }
            None => {}
        }
    }
}
