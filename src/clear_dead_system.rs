use super::{State,Name,Statistics,Player};
use bracket_lib::terminal::console;
use hecs::Entity;
pub struct ClearDeadSystem
{
}

impl ClearDeadSystem
{
    /// Does not currently do anything when the player is killed -- remember to change this nerd!
    pub fn run(state : &mut State)
    {
        let mut entities_to_despawn : Vec<Entity> = Vec::new();
        for (_id,(stats,name,player))
         in state.world.query_mut::<(&Statistics,&Name,Option<&Player>)>()
         {
            if stats.hp <= 0
            {
                match player
                {
                    Some(_p) => 
                    {
                        console::log("You have died!!!!!");
                    }
                    None => 
                    {
                        console::log(format!("The {} dies!",name.name));
                        entities_to_despawn.push(_id);
                    }
                }
            }
         }
         for entity in entities_to_despawn.iter()
         {
            state.world.despawn(*entity).expect("Couldn't successfully despawn dead entity!");
         }

    }


}
