use crate::{components, statistics::{self, Pools}, Equipped, InContainer, Position, ProgramState};

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
        let mut entities_to_despawn : Vec<(Entity, Position)> = Vec::new();
        for (_id,(stats,name,player, pos))
         in state.world.query_mut::<(&Pools,&Name,Option<&Player>, &Position)>()
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
                        entities_to_despawn.push((_id, pos.clone()));
                    }
                }
            }
         }

         let pool = state.world.query_one_mut::<&mut Pools>(state.player_ent.unwrap()).unwrap();
         pool.exp += xp_to_award;

         statistics::check_level_up(state);

         for (entity, pos) in entities_to_despawn.iter()
         {
            let eq_items = state.world.query::<(&components::Item, &Equipped)>()
                .iter().filter(|(_ent,(_item, equipped))| equipped.owner == *entity)
                .map(|ent| ent.0).collect::<Vec<_>>();

                let bp_items = state.world.query::<(&components::Item, &InContainer)>()
                .iter().filter(|(_ent,(_item, bp))| bp.owner == *entity)
                .map(|ent| ent.0).collect::<Vec<_>>();

            for item in eq_items.iter()
            {
                console::log(format!("removing items! pos: {}:{}", pos.x, pos.y));
                state.world.remove_one::<Equipped>(*item)
                    .expect("Couldn't remove Equipped from item to drop from dead mob!");
                
                state.world.insert_one(*item, Position{x : pos.x, y: pos.y}).expect("Couldn't insert position into item to drop from mob!");
            }

            for item in bp_items.iter()
            {
                state.world.remove_one::<InContainer>(*item)
                    .expect("Couldn't remove inContainer from item to drop from dead mob!");
                
                state.world.insert_one(*item, pos.clone()).expect("Couldn't insert position into item to drop from mob!");
            }



            //delete entity
            state.world.despawn(*entity).expect("Couldn't successfully despawn dead entity!");
         }

    }


}
