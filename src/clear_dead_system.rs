use crate::{statistics::{self, Pools}, ProgramState};

use super::{State,Name, Player};
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
        let mut xp_to_award = 0;
        let mut entities_to_despawn : Vec<Entity> = Vec::new();
        for (_id,(stats,name,player))
         in state.world.query_mut::<(&Pools,&Name,Option<&Player>)>()
         {
            if stats.hitpoints.current_value <= 0
            {
                match player
                {
                    Some(_p) => 
                    {
                        console::log("You have died!!!!!");
                        state.current_state = ProgramState::GameOver;
                    }
                    None => 
                    {
                        let xp = statistics::monster_xp_drop(stats.level);
                        state.game_log.add_log(format!("The {} dies! You get {} xp!", name.name, xp ));

                        console::log(format!("The {} dies!",name.name));

                        xp_to_award += xp; 
                        entities_to_despawn.push(_id);
                    }
                }
            }
         }

         let pool = state.world.query_one_mut::<&mut Pools>(state.player_ent.unwrap()).unwrap();
         pool.exp += xp_to_award;

         statistics::check_level_up(state);

         for entity in entities_to_despawn.iter()
         {
            state.world.despawn(*entity).expect("Couldn't successfully despawn dead entity!");
         }

    }


}
