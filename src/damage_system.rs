use hecs::Entity;
use bracket_lib::terminal::console;
use super::{State,TakeDamage,Statistics,Name};



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
        let mut dmg_comps_to_remove : Vec<Entity> = Vec::new();
        for  (id,(dmg_to_take,stats,name))
         in state.world.query_mut::<(&TakeDamage,&mut Statistics, &Name)>()
        {
            //console::log("aaaaaaaaaaaaaaa");
            dmg_comps_to_remove.push(id);
            for dmg in dmg_to_take.damage_to_take.iter()
            {
                stats.hp -= dmg;
                state.game_log.add_log(format!("{} took {} damage!",name.name,dmg));
                console::log(format!("{} took {} damage!",name.name,dmg));
            }
        }

        for  dmg_comp in dmg_comps_to_remove.iter()
        {
            state.world.remove_one::<TakeDamage>(*dmg_comp).expect("Couldn't remove damage comp from entity!");
        }
    }
}