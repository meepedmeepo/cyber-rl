use bracket_lib::prelude::console;

use crate::{
    map_indexing::SPATIAL_INDEX, HasMoved, Hidden, Map, Name, Position, State, Trigger,
    TriggerOnEnter, Triggered,
};

pub fn run(state: &mut State) {
    let mut queried_ents = Vec::new();
    let mut triggered_props = Vec::new();
    for (ent, (pos, name, _moved)) in state.world.query_mut::<(&Position, &Name, &HasMoved)>() {
        queried_ents.push(ent);
        //console::log(format!("{} traps on map", state.map.props.len()));
        let idx = state.map.xy_idx(pos.x, pos.y) as i32;

        match SPATIAL_INDEX.lock().unwrap().get_prop_entity_at_idx(idx) {
            Some(prop) => {
                //console::log("trap triggered started!");
                triggered_props.push((prop, ent, idx));
            }

            None => {}
        }
    }

    for (prop, target, idx) in triggered_props.iter() {
        if state.world.get::<&TriggerOnEnter>(*prop).is_ok()
            && state.world.get::<&Trigger>(*prop).is_ok()
        {
            let _ = state.world.remove_one::<Hidden>(*prop);
            //console::log("trap triggered!");
            let _ = state.world.insert_one(
                *prop,
                Triggered {
                    entity: *target,
                    idx: *idx as i32,
                },
            );
        }
    }

    for ent in queried_ents.iter() {
        state
            .world
            .remove_one::<HasMoved>(*ent)
            .expect("Couldn't remove HasMoved component from entity!");
    }
}
