use bracket_lib::terminal::console;
use hecs::{World, Entity};
use crate::{damage_system::DamageSystem, CombatStats, Statistics};

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
        let mut defenders_to_damage : Vec<(Entity,i32)> = Vec::new();
        
        for (_id,(attack,_name, stats)) 
        in state.world.query::<(&mut Attack,&Name,&CombatStats)>().iter()
        {
            attackers.push(_id);
            defenders_to_damage.push((attack.target,stats.power.total));
        }

        for (target,dmg) in defenders_to_damage
        {
            DamageSystem::mark_for_damage(state, target, dmg);
        }


        for entity in attackers.iter()
        {
            state.world.remove_one::<Attack>(*entity)
            .expect("Couldn't remove Attack component from the attacker");
        }
    }



}