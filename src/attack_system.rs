use bracket_lib::terminal::console;
use hecs::{World, Entity};
use crate::{Statistics};

use super::{State, Attack, Name, TakeDamage};
pub struct AttackSystem
{}

impl AttackSystem
{
    pub fn add_attack(attacker : hecs::Entity, target : hecs::Entity, state : &mut State)
    {
        //console::log("Inserting attack component");
        state.world.insert_one(attacker, Attack{target: target})
        .expect("Failed at trying to insert attack component!\n");
    }

    /// TODO: this will handle the logic of checking if an attack hits but for now just directly creates suffer TakeDamage
    pub fn run(state : &mut State)
    {
        let mut attackers: Vec<Entity> = Vec::new();
        let mut defendersToDamage : Vec<(Entity,i32)> = Vec::new();
        for (_id,(attack,_name, stats)) 
        in state.world.query::<(&mut Attack,&Name,&Statistics)>().iter()
        {
            attackers.push(_id);
            let query = state.world.get::<&mut TakeDamage>(attack.target);
            match query 
            {
                Ok(mut res)   => {res.damage_to_take.push(stats.strength);}
                Err(_) => 
                {
                    //console::log("Gonna need to add a new damage comp");
                    defendersToDamage.push((attack.target,stats.strength));
                    //state.world.insert_one(attack.target, TakeDamage{damage_to_take: vec![stats.strength;1]});
                } 
            }
            //state.world.insert_one(attack.target, )
        }
        for (target,dmg) in defendersToDamage.iter()
        {
            state.world.insert_one(*target, TakeDamage{damage_to_take: vec![*dmg]})
            .expect("Failed to add TakeDamage component to attack target!");
        }

        for entity in attackers.iter()
        {
            state.world.remove_one::<Attack>(*entity).expect("Couldn't remove Attack component from the attacker");
        }
    }



}