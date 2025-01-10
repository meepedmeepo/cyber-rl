mod initiative;
pub use initiative::*;
mod adjacent_ai;
pub use adjacent_ai::*;
use crate::{statistics::BaseStatistics, State};


pub struct MyTurn
{
}


pub struct Energy
{
    pub value: i32,
}

pub enum ActionType
{
    Move,
    Attack,
    Equip
}

impl ActionType
{
    pub fn get_cost(&self) -> i32
    {
        match *self
        {
            ActionType::Move => 100,
            ActionType::Equip => 200,
            ActionType::Attack => 150,
        }
    }
}

pub fn world_tick(state : &mut State)
{
    //increase world turn timer by one
    state.turn_number += 1;
    //select which entities will have turns
   // init_turn_queue(state);


}