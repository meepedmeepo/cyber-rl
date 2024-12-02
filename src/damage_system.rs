use hecs::Entity;
use bracket_lib::terminal::console;
use super::{State,TakeDamage,Statistics,Name};



pub struct DamageSystem
{}


impl DamageSystem
{
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
                console::log(format!("{} took {} damage!",name.name,dmg));
            }
        }

        for  dmg_comp in dmg_comps_to_remove.iter()
        {
            state.world.remove_one::<TakeDamage>(*dmg_comp).expect("Couldn't remove damage comp from entity!");
        }
    }
}