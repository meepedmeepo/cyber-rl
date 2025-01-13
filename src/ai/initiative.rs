use std::sync::Mutex;

use crate::{statistics::BaseStatistics, Position, ProgramState, State};

use super::{ Energy, MyTurn};
use bracket_lib::prelude::Point;
use hecs::Entity;
use priority_queue::{self, PriorityQueue};

lazy_static!{
    pub static ref TURN_QUEUE : Mutex<PriorityQueue<Entity, i32>> = Mutex::new(PriorityQueue::new());
}

pub fn peek_turn_queue() -> Option<(Entity, i32)>
{
    TURN_QUEUE.lock().unwrap().pop()
}

pub fn push_turn_queue(ent : Entity, energy: i32)
{
    TURN_QUEUE.lock().unwrap().push(ent, energy);
}

pub fn init_turn_queue(state : &mut State)
{
    TURN_QUEUE.lock().unwrap().clear();

    let mut p:PriorityQueue<Entity, i32> = PriorityQueue::new();
    
    //add energy, check if has enough energy and isn't too far from player and if both are true adds them to the turn order
    //for this tick
    for (ent, (energy, pos, stats)) 
        in state.world.query_mut::<(&mut Energy, &Position, &BaseStatistics)>()
    {

        if bracket_lib::geometry::DistanceAlg::Pythagoras
            .distance2d(state.player_pos, Point::new(pos.x, pos.y)) < 25. 
        {
            let mut energy_gain = 100;
            energy_gain += stats.dexterity.get_modifier() * 10;
            energy.value += energy_gain;

            if energy.value > 0
            {
                p.push(ent, energy.value);
            }
        }
    }

    TURN_QUEUE.lock().unwrap().append(&mut p); 
}


///If an entity has more than 0 energy it is given a chance to have a turn - will process all available MyTurns and when there are none
/// left all entities will have energy added to them. If the player can act then the program state will go to AwaitingInput
pub fn run_initiative(state : &mut State) -> ProgramState
{

    if state.world.query_mut::<&MyTurn>().into_iter().len() < 1
    {
        let mut turns_to_add = Vec::new();
        
        for (ent, (energy, stats, pos)) 
            in state.world.query_mut::<(&mut Energy, &BaseStatistics, &Position)>()
        {
            if bracket_lib::geometry::DistanceAlg::Pythagoras
                .distance2d(state.player_pos, Point::new(pos.x, pos.y)) < 30. || ent == state.player_ent.unwrap()
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