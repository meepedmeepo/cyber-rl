use crate::{HasMoved, Hidden, Map, Name, Position, State, TriggerOnEnter, Triggered};



pub fn run(state: &mut State)
{
    let mut queried_ents = Vec::new();
    let mut triggered_props = Vec::new();
    for (ent, (pos, name)) 
        in state.world.query_mut::<(&Position, &Name)>().with::<&HasMoved>()
    {
        queried_ents.push(ent);

        let idx = Map::xy_id(pos.x, pos.y);

        match state.map.props.get(&(idx as i32))
        {
            Some(prop) =>
            {
                triggered_props.push((*prop,ent));
            }

            None => {}
        }
    }

    for (prop, target) in triggered_props.iter()
    {
        if state.world.get::<&TriggerOnEnter>(*prop).is_ok()
        {
            let _ = state.world.remove_one::<&Hidden>(*prop);

            let _ = state.world.insert_one(*prop, Triggered{entity: *target});
        }
    }



    for ent in queried_ents.iter()
    {
        state.world.remove_one::<HasMoved>(*ent).expect("Couldn't remove HasMoved component from entity!");
    }
}