use std::{collections::HashMap, hash::Hash};

use bracket_lib::prelude::Point;
use hecs::Entity;

use crate::{raws::{self, Reaction}, Faction, FoV, Map, Player, Position, State, WantsToApproach, WantsToFlee, MAPWIDTH};

use super::MyTurn;




pub fn visible_ai_system(state :&mut State)
{
    //stores possible targets for each entity that can see enemies
    let mut possible_attacks: HashMap<Entity,Vec<usize>> = HashMap::new();

    //stores lists of indices to flee from for targets that want to flee
    let mut flee_targets : HashMap<Entity, Vec<usize>> = HashMap::new();

    for (ent, (_turn, my_faction, pos, fov)) 
        in state.world.query::<(&MyTurn, &Faction, &Position, &FoV)>().without::<&Player>().iter()
    {
        let my_idx = Map::xy_id(pos.x, pos.y);
        let mut reactions : Vec<(usize, Reaction)> = Vec::new();
        let mut flee : Vec<usize> = Vec::new();

        for tile in fov.visible_tiles.iter()
        {
            let idx = Map::xy_id(tile.x, tile.y);
            if idx != my_idx
            {
                evaluate(idx, state, &my_faction.name, &mut reactions);
            }
        }

        let mut done = false;
        let mut attacks = Vec::new();
        for reaction in reactions.iter()
        {
            match reaction.1 
            {
                Reaction::Attack => {attacks.push(reaction.0); done = true;},
                Reaction::Flee => flee.push(reaction.0),
                _ => {}
            }
        }

        possible_attacks.insert(ent, attacks);

        if !done && !flee.is_empty()
        {
            flee_targets.insert(ent, flee);
        }
    }

    for (id, targets) in possible_attacks.iter()
    {
        let index = state.rng.range(0, targets.len());

        let _ = state.world.insert_one(*id, WantsToApproach{target: targets[index] as i32});
    }

    for (id, tiles) in flee_targets.iter()
    {
        let _ = state.world.insert_one(*id, WantsToFlee {indices : tiles.clone()});
    }
}

fn evaluate(idx : usize, state : &State , my_faction : &str, reactions : &mut Vec<(usize, raws::Reaction)>)
{
    for ent in state.map.get_mob_entities_at_position(state
        , Point::new(idx as i32 % MAPWIDTH , idx as i32 / MAPWIDTH))
    {
        if let Ok(faction) = state.world.get::<&Faction>(ent)
        {
            reactions.push((idx, raws::faction_reaction
                (my_faction, &faction.name, &raws::RAWS.lock().unwrap())));
        }

    }
}