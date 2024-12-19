use bracket_lib::{color::{BLACK, RGB, WHITE}, terminal::console};
use hecs::{World, Entity};
use crate::{damage_system::DamageSystem, CombatStats, Position, Statistics};

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
        let mut attackers: Vec<(Entity, Position)> = Vec::new();
        let mut defenders_to_damage : Vec<(Entity,i32)> = Vec::new();
        
        for (_id,(attack,_name, stats, pos)) 
        in state.world.query::<(&mut Attack,&Name,&CombatStats, &Position)>().iter()
        {
            attackers.push((_id, *pos));
            defenders_to_damage.push((attack.target,stats.power.total));
        }

        for (target,dmg) in defenders_to_damage
        {
            DamageSystem::mark_for_damage(state, target, dmg);
        }


        for (entity, pos) in attackers.iter()
        {
            state.world.remove_one::<Attack>(*entity)
                .expect("Couldn't remove Attack component from the attacker");

            state.particle_builder.request(pos.x, pos.y,
                 RGB::named(WHITE), RGB::named(BLACK), '/', 50.);
        }
    }



}