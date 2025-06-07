mod initiative;
use std::collections::VecDeque;

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
mod default_move_ai;
pub use default_move_ai::*;
mod idle_movement;
pub use idle_movement::*;
mod quipping;
pub use quipping::*;
mod spot_traps;
use spot_traps::*;
mod chase_ai;
pub use chase_ai::*;
mod pathing;
pub use pathing::*;

use crate::{statistics::BaseStatistics, State};

pub struct MyTurn {}

pub struct Energy {
    pub value: i32,
}

pub enum ActionType {
    Move,
    Attack,
    Equip,
    Pickup,
    UseItem,
    OpenDoor,
}

impl ActionType {
    pub fn get_cost(&self) -> i32 {
        match *self {
            ActionType::Move => 80,
            ActionType::Equip => 200,
            ActionType::Attack => 100,
            ActionType::Pickup => 50,
            ActionType::OpenDoor => 50,
            ActionType::UseItem => 100,
        }
    }
}

pub fn apply_energy_cost(state: &mut State, action: ActionType, ent: Entity) {
    if let Ok(mut energy) = state.world.get::<&mut Energy>(ent) {
        energy.value -= action.get_cost();
    }
}

pub struct NavPath {
    path: VecDeque<usize>,
    idle_time: u32,
}

impl NavPath {
    pub fn new(path: VecDeque<usize>) -> Self {
        NavPath {
            path,
            idle_time: 0u32,
        }
    }

    pub fn get_next(&mut self) -> Option<usize> {
        self.path.pop_front()
    }

    pub fn add_waypoint(&mut self, idx: usize) {
        self.path.push_front(idx);
    }
}
