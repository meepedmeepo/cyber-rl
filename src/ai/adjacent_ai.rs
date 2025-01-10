
use bracket_lib::prelude::Point;
use hecs::Entity;

use crate::{raws::{self, Reaction}, Attack, Faction, Map, Player, Position, State, MAPHEIGHT, MAPWIDTH};

use super::MyTurn;





pub fn adjacent_ai_system(state : &mut State)
{
    let mut turn_done: Vec<Entity> = Vec::new();
    let mut attacks : Vec<(Entity, Entity)> = Vec::new();

    for (ent, (_turn, faction, pos)) 
        in state.world.query::<(&MyTurn, &Faction, &Position)>().without::<&Player>().iter()
    {
        
        let mut reactions: Vec<(Entity, raws::Reaction)> = Vec::new();
        let my_faction = &faction.name;
        let idx = Map::xy_id(pos.x, pos.y);
        let w = MAPWIDTH;
        let h = MAPHEIGHT;

        //checking possible reactions for each of the 8 cardinal directions
        if pos.x > 0 {evaluate(idx-1, state, my_faction, &mut reactions);}
        if pos.x < w-1 {evaluate(idx+1, state, my_faction, &mut reactions);}
        if pos.y > 0 {evaluate(idx - w as usize, state, my_faction, &mut reactions);}
        if pos.y < h-1 {evaluate(idx + w as usize, state, my_faction, &mut reactions);}
        if pos.y > 0 && pos.x > 0 {evaluate((idx - w as usize) -1, state, my_faction, &mut reactions);} 
        if pos.y > 0 && pos.x < w - 1 {evaluate((idx - w as usize) + 1, state, my_faction, &mut reactions);}
        if pos.y < h-1 && pos.x > 0 {evaluate((idx + w as usize) - 1, state, my_faction, &mut reactions);}
        if pos.y < h - 1 && pos.x < w - 1 {evaluate((idx + w as usize) + 1, state, my_faction, &mut reactions);}

        let mut done = false;
        for reaction in reactions.iter()
        {
            if let Reaction::Attack = reaction.1
            {
                done = true;
                attacks.push((ent, reaction.0));
            }
        }

        if done == true
        {
            turn_done.push(ent);
        }
    }
    //todo: change this so that a random target is selected from possible enemies if there are several valid targets for an entity!
    for attack in attacks.iter()
    {
        let _ =state.world.insert_one(attack.0, Attack{target: attack.1});
    }

    for done in turn_done.iter()
    {
        let _ = state.world.remove_one::<MyTurn>(*done);
    }
}


fn evaluate(idx : usize, state : &State , my_faction : &str, reactions : &mut Vec<(Entity, raws::Reaction)>)
{
    for ent in state.map.get_mob_entities_at_position(state
        , Point::new(idx as i32 % MAPWIDTH , idx as i32 / MAPWIDTH))
    {
        if let Ok(faction) = state.world.get::<&Faction>(ent)
        {
            reactions.push((ent, raws::faction_reaction
                (my_faction, &faction.name, &raws::RAWS.lock().unwrap())));
        }

    }
}