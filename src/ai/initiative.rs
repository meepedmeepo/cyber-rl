use crate::{hunger::hunger_system, statistics::BaseStatistics, time_system, Map, Position, ProgramState, State};

use super::{ spot_traps, Energy, MyTurn};
use bracket_lib::prelude::Point;



///If an entity has more than 0 energy it is given a chance to have a turn - will process all available MyTurns and when there are none
/// left all entities will have energy added to them. If the player can act then the program state will go to AwaitingInput
pub fn run_initiative(state : &mut State) -> ProgramState
{
    if state.world.query_mut::<&MyTurn>().into_iter().len() < 1
    {
        hunger_system(state);
        time_system::time_system(state);
        spot_traps(state);

        let mut turns_to_add = Vec::new();
        
        for (ent, (energy, stats, pos)) 
            in state.world.query_mut::<(&mut Energy, &BaseStatistics, &Position)>()
        {
            if bracket_lib::geometry::DistanceAlg::Pythagoras
                .distance2d(state.player_pos, Point::new(pos.x, pos.y)) < 30. || ent == state.player_ent.unwrap()
                || state.map.visible_tiles[state.map.xy_idx(pos.x, pos.y)]
            {
                let mut energy_gain = 50;
                energy_gain += std::cmp::max(0,stats.dexterity.get_modifier() * 5);
                energy.value += energy_gain;
            
                if energy.value > 0
                {
                    turns_to_add.push(ent);
                }
            }
        }

        for t in turns_to_add.iter()
        {
            let _ = state.world.insert_one(*t, MyTurn{});
        }

        if let Ok(_) = state.world.get::<&MyTurn>(state.player_ent.unwrap())
        {
            return ProgramState::AwaitingInput;
        }
    }

    ProgramState::Ticking
}