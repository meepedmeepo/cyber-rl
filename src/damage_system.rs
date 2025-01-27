use hecs::Entity;
use bracket_lib::{color::{RED, RGB, WHITE}, prelude::to_cp437, terminal::console};
use crate::{statistics::Pools, Position};

use super::{State,TakeDamage,Name};



pub struct DamageSystem
{}


impl DamageSystem
{
    pub fn mark_for_damage(state : &mut State, target : Entity, dmg : i32)
    {
        let query = state.world.get::<&mut TakeDamage>(target);
        
        match query
        {
            Ok(mut dmg_comp) =>
            {
                dmg_comp.damage_to_take.push(dmg);
            }

            Err(_) =>
            {
                std::mem::drop(query);
                state.world.insert_one(target, TakeDamage{damage_to_take: vec![dmg]})
                .expect("Couldn't insert TakeDamage component!");
            }
        }
    }
    
pub fn run(state : &mut State)
    {
        let mut dmg_comps_to_remove : Vec<(Entity, Position)> = Vec::new();
        for  (id,(dmg_to_take,stats,name, pos))
            in state.world.query_mut::<(&TakeDamage,&mut Pools, &Name, &Position)>()
        {
            //console::log("aaaaaaaaaaaaaaa");
            dmg_comps_to_remove.push((id, *pos));
            for dmg in dmg_to_take.damage_to_take.iter()
            {
                //let adjusted_dmg = std::cmp::max(1,*dmg);
                stats.hitpoints.damage(*dmg);
                state.game_log.add_log(format!("{} took {} damage!",name.name, *dmg));
                console::log(format!("{} took {} damage!",name.name, *dmg));
            }
        }

        for  (dmg_comp, pos) in dmg_comps_to_remove.iter()
        {
            state.world.remove_one::<TakeDamage>(*dmg_comp).expect("Couldn't remove damage comp from entity!");

            state.particle_builder.request(pos.x, pos.y, RGB::named(WHITE), RGB::named(RED),
                to_cp437('!'), 200., Some(*dmg_comp));
        }
    }
}