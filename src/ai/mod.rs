mod initiative;
use hecs::Entity;
pub use initiative::*;
mod adjacent_ai;
pub use adjacent_ai::*;
mod visible_ai;
pub use visible_ai::*;
mod approach_ai;
pub use approach_ai::*;
mod flee_ai;
pub use flee_ai::*;
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

pub fn apply_energy_cost (state: &mut State, action: ActionType, ent : Entity)
{
    if let Ok( mut energy) = state.world.get::<&mut Energy>(ent)
    {
        energy.value -= action.get_cost();
    }
}