use hecs::Entity;

use crate::{FoV, HasMoved, Map, Position, State, WantsToApproach, MAPWIDTH};

use super::{apply_energy_cost, MyTurn};





pub fn approach_ai_system(state : &mut State)
{
    let mut turn_done: Vec<Entity> = Vec::new();
    let mut has_moved: Vec<Entity> = Vec::new();
    
    for (ent, (_turn, approach, fov, pos)) 
        in state.world.query_mut::<(&MyTurn, &WantsToApproach, &mut FoV, & mut Position)>()
    {
        turn_done.push(ent);

        let path = bracket_lib::pathfinding::a_star_search
            (Map::xy_id(pos.x, pos.y) as i32
            , Map::xy_id(approach.target % MAPWIDTH, approach.target / MAPWIDTH) as i32, &state.map);

        if path.success && path.steps.len() > 1
        {
            let mut idx = Map::xy_id(pos.x, pos.y);
            state.map.blocked[idx] = false;

            pos.x = path.steps[1] as i32 % MAPWIDTH;
            pos.y = path.steps[1] as i32 / MAPWIDTH;

            has_moved.push(ent);

            idx = Map::xy_id(pos.x, pos.y);
            state.map.blocked[idx] = true;

            fov.dirty = true;
        }
    }

    for moved in has_moved.iter()
    {
        let _ = state.world.insert_one(*moved, HasMoved{});
        
        apply_energy_cost(state, super::ActionType::Move, *moved);
    }

    for done in turn_done.iter()
    {
        let _  = state.world.remove_one::<WantsToApproach>(*done);
        let _ = state.world.remove_one::<MyTurn>(*done);
    }
}