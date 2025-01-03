use std::{collections::VecDeque, sync::Mutex};

use hecs::Entity;

use crate::State;



lazy_static!
{
    pub static ref EFFECTQUEUE : Mutex<VecDeque<EffectSpawner>> = Mutex::new(VecDeque::new());
}

#[derive(Debug, PartialEq, Eq)]
pub enum EffectType
{
    Damage {amount : i32}
}

#[derive(Clone, PartialEq, Eq)]
pub enum Targets
{
    Single {target : Entity},
    Area {target: Vec<Entity>},

}

pub struct EffectSpawner
{
    pub creator: Option<Entity>,
    pub effect_type: EffectType,
    pub targets : Targets
}

pub fn add_effect(creator : Option<Entity>, effect_type: EffectType, targets: Targets)
{
    EFFECTQUEUE
        .lock()
        .unwrap()
        .push_back(EffectSpawner { creator, effect_type, targets });
}

pub fn run_effect_queue(state: &mut State)
{
    loop
    {
        let effect = EFFECTQUEUE.lock().unwrap().pop_front();
        if let Some(effect) = effect
        {

        } else 
        {
            break;   
        }
    }
}


fn target_applicator(state: &mut State, effect: &EffectSpawner)
{
    
}