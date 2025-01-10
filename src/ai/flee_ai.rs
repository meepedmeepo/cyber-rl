use bracket_lib::prelude::DijkstraMap;
use hecs::Entity;

use crate::{FoV, HasMoved, Map, Player, Position, State, WantsToApproach, WantsToFlee, MAPHEIGHT, MAPWIDTH};

use super::{apply_energy_cost, MyTurn};




pub fn flee_ai_system(state : &mut State)
{
    let mut turn_done: Vec<Entity> = Vec::new();
    let mut has_moved: Vec<Entity> = Vec::new();

    for (ent, (pos, flee, fov, _turn)) 
        in state.world.query_mut::<(&mut Position, &WantsToFlee, &mut FoV, &MyTurn)>().without::<&Player>()
    {
        turn_done.push(ent);

        let my_idx = Map::xy_id(pos.x, pos.y);
        state.map.populate_blocked();

        let flee_map = bracket_lib::pathfinding::DijkstraMap::new(MAPWIDTH as usize
            , MAPHEIGHT as usize, &flee.indices, &state.map, 100.0);

        let flee_target = DijkstraMap::find_highest_exit(&flee_map, my_idx, &state.map);

        if let Some(flee_target) = flee_target
        {
            if !state.map.blocked[flee_target as usize]
            {
                state.map.blocked[my_idx] = false;
                state.map.blocked[flee_target as usize] = true;

                fov.dirty = true;
                pos.x = flee_target as i32 % MAPWIDTH;
                pos.y = flee_target as i32 / MAPWIDTH;
                has_moved.push(ent);
            }
        }
    }
    for moved in has_moved.iter()
    {
        let _ = state.world.remove_one::<HasMoved>(*moved);
        
        apply_energy_cost(state, super::ActionType::Move, *moved);
    }

    for done in turn_done.iter()
    {
        let _ = state.world.remove_one::<MyTurn>(*done);
        let _ = state.world.remove_one::<WantsToFlee>(*done);
    }
    
}