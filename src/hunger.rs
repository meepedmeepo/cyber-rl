use bracket_lib::prelude::console;

use crate::{damage_system::DamageSystem, effects::{add_effect, EffectType, Targets}, statistics::{BaseStatistics, Pools, StatPool}, Name, State, WantsToRest};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HungerState
{
    Satiated,
    WellFed,
    Fed,
    Hungry,
    Starving
}
#[derive(Clone, Copy)]
pub struct HungerLevel
{
   pub nutrition: StatPool,
}
impl HungerLevel
{
    pub fn get_hunger_state(&self) -> HungerState
    {
        let percent = (self.nutrition.current_value as f32/ self.nutrition.max_value as f32) *100.;
        if percent >= 85.
        {
            return HungerState::Satiated;
        }else if percent >= 65.
        {
            return HungerState::WellFed;
        }else if percent >= 40.
        {
            return  HungerState::Fed;
        }else if percent >= 15.
        {
            return HungerState::Hungry;
        }else {
            return HungerState::Starving;
        }
    }
}


pub fn hunger_system(state: &mut State)
{
    let res =state.turn_number % 5;
    for (_id, (hunger,pools, stats, name, is_resting)) 
        in state.world.query_mut::<(&mut HungerLevel, &mut Pools, &BaseStatistics, &Name, Option<&WantsToRest>)>()
    {
        if let Some(_resting) = is_resting
        {
            if hunger.get_hunger_state() != HungerState::Starving && hunger.get_hunger_state() != HungerState::Hungry
            {
                //TODO: CHECK IF IN COMBAT FIRST MAYBE??
                if pools.hitpoints.current_value != pools.hitpoints.max_value
                {
                    //pools.hitpoints.restore(2);
                    add_effect(None, EffectType::Healing { amount: 1 }, Targets::Single { target: _id });
                    hunger.nutrition.damage(5);
                }
            }
        }


        if res == 0
        {

            hunger.nutrition.damage(3);

            if hunger.get_hunger_state() == HungerState::Starving
            {
                if hunger.nutrition.current_value > 0
                {
                    let msg = format!("{} took damage from starvation!", name.name.clone());
                    state.game_log.add_log(msg.clone());
                    console::log(msg);

                    
                    add_effect(None, EffectType::Damage { amount: 3 }, Targets::Single { target: _id })
                }else
                {
                    let msg = format!("{} died of starvation!", name.name.clone());
                    state.game_log.add_log(msg.clone());
                    console::log(msg);

                    pools.hitpoints.damage(10000);
                    //TODO: add a insta kill EffectType
                }
            }
        }


    }
}