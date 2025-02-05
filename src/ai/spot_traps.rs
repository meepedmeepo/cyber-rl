use crate::{components::{FoV, Hidden}, statistics::BaseStatistics, State};



///ran each turn when initiative is calculated - currently only the player is able to reveal traps and they will be revealed for all entities
pub fn spot_traps(state : &mut State)
{
    let query = state.world.query_one_mut::<&FoV>(state.player_ent.unwrap()).unwrap();
    let fov = query.clone();

    for tile in fov.visible_tiles.iter()
    {
        let idx = state.map.xy_idx(tile.x, tile.y) as i32;

        match state.map.props.get(&idx)
        {
            Some(prop) =>
            {
                if state.world.get::<&Hidden>(*prop).is_ok()
                {
                    let roll = state.rng.roll_dice(1, 20);
                    let bonus = state.world.query_one_mut::<&BaseStatistics>(state.player_ent.unwrap()).unwrap().intelligence.get_modifier();
                    if roll + bonus > 17 
                    {
                        let _ = state.world.remove_one::<Hidden>(*prop);
                    }
                }
            }
            None => {}
        }

    }
}