use bracket_lib::prelude::a_star_search;
use hecs::Entity;

use crate::{components::{HasMoved, MovementType}, Position, State};

use super::{apply_energy_cost, MyTurn};




pub fn idle_movement_ai(state : &mut State)
{
    let mut moved: Vec<Entity> = Vec::new();
    for (ent, (movement, pos, _turn)) in state.world.query_mut::<(&mut MovementType,&mut Position,&MyTurn)>()
    {
        if let MovementType::RandomWaypoint {path } = movement
        {
            if path.is_none()
            {
                let mut attempt = 5;
                while  attempt > 0
                {
                    let roll = state.rng.random_slice_index(&state.map.map);

                    match roll
                    {
                        Some(idx) => 
                        {
                            if !state.map.blocked[idx]
                            {
                                let my_idx = state.map.xy_idx(pos.x, pos.y);
                                let p = a_star_search(my_idx, idx, &state.map);
                                if p.success && p.steps.len() > 8
                                {
                                    *path = Some((p.steps
                                    .into_iter().rev().collect(), 0usize));

                                    break;
                                } else {
                                    attempt -= 1;
                                }
                            }
                            else 
                            {
                                attempt -= 1;   
                            }
                        }
                        None => { attempt -= 1;}
                    }
                }
            }

            if let Some((p, index)) = path
            {
                if p.len() -1  < *index
                {
                    *path = None;
                    
                } 
                else if !state.map.blocked[p[*index]] 
                {
                    let my_idx = state.map.xy_idx(pos.x, pos.y);
                    let idx = p[*index];
                    let x = idx % state.map.map_width as usize;
                    let y = idx / state.map.map_width as usize;
                    pos.x = x as i32;
                    pos.y = y as i32;

                    state.map.blocked[my_idx] = false;
                    state.map.blocked[idx] = true;

                    moved.push(ent);
                    
                    *index += 1;
                }
            };
        }
    }

    for ent in moved.iter()
    {
        let _ = state.world.insert_one(*ent, HasMoved{});
        let _ = state.world.remove_one::<MyTurn>(*ent);

        apply_energy_cost(state, super::ActionType::Move, *ent);
    }


}