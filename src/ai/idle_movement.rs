use bracket_lib::prelude::a_star_search;
use hecs::Entity;

use crate::{components::MovementType, Position, State};

use super::MyTurn;




pub fn idle_movement_ai(state : &mut State)
{
    let mut to_move: Vec<(Entity, usize)> = Vec::new();
    for (ent, (movement, pos, _turn)) in state.world.query_mut::<(&mut MovementType,&Position,&MyTurn)>()
    {
        if let MovementType::RandomWaypoint {path } = movement
        {
            if path.is_none()
            {
                let mut attempt = 3;
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
                                *path = Some(a_star_search(my_idx, idx, &state.map).steps
                                    .into_iter().rev().collect());

                                break;
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

            if path.is_some()
            {
                
            }
        }
    }
}