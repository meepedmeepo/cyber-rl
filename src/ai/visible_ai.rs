use std::collections::HashMap;

use bracket_lib::prelude::Point;
use hecs::Entity;

use crate::{
    map_indexing::SPATIAL_INDEX,
    raws::{self, Reaction},
    statistics::Pools,
    utils::{get_mob_entities_at_position, get_mobs_at_idx},
    Faction, FoV, Map, Player, Position, State, WantsToApproach, WantsToFlee,
};

use super::{InCombat, MyTurn};

pub fn visible_ai_system(state: &mut State) {
    //stores possible targets for each entity that can see enemies
    let mut possible_attacks: HashMap<Entity, Vec<(usize, Entity)>> = HashMap::new();

    //stores lists of indices to flee from for targets that want to flee
    let mut flee_targets: HashMap<Entity, Vec<usize>> = HashMap::new();

    for (ent, (_turn, my_faction, pos, fov)) in state
        .world
        .query::<(&MyTurn, &Faction, &Position, &FoV)>()
        .without::<&Player>()
        .iter()
    {
        let my_idx = state.map.xy_idx(pos.x, pos.y);
        let mut reactions: Vec<(usize, Reaction, Entity)> = Vec::new();
        let mut flee: Vec<usize> = Vec::new();

        for tile in fov.visible_tiles.iter() {
            let idx = state.map.xy_idx(tile.x, tile.y);
            if idx != my_idx {
                evaluate(idx, state, &my_faction.name, &mut reactions);
            }
        }

        let mut done = false;
        let mut attacks = Vec::new();
        for reaction in reactions.iter() {
            match reaction.1 {
                Reaction::Attack => {
                    attacks.push((reaction.0, reaction.2));
                    done = true;
                }
                Reaction::Flee => flee.push(reaction.0),
                _ => {}
            }
        }

        possible_attacks.insert(ent, attacks);

        if !done && !flee.is_empty() {
            flee_targets.insert(ent, flee);
        }
    }

    for (id, targets) in possible_attacks.iter() {
        let mut index: usize = 0;
        if targets.len() == 0 {
            continue;
        }
        if targets.len() == 1 {
            index = 0;
        } else {
            index = state.rng.range(0, targets.len());
        }

        let _ = state.world.insert_one(
            *id,
            WantsToApproach {
                target: targets[index].0 as i32,
            },
        );

        let _ = state.world.insert_one(
            *id,
            InCombat {
                target: targets[index].1,
            },
        );
    }

    for (id, tiles) in flee_targets.iter() {
        let _ = state.world.insert_one(
            *id,
            WantsToFlee {
                indices: tiles.clone(),
            },
        );
        //let _ = state.world.remove_one::<MyTurn>(*id);
    }
}

fn evaluate(
    idx: usize,
    state: &State,
    my_faction: &str,
    reactions: &mut Vec<(usize, raws::Reaction, Entity)>,
) {
    for ent in get_mobs_at_idx(state, idx) {
        if let Ok(faction) = state.world.get::<&Faction>(ent) {
            reactions.push((
                idx,
                raws::faction_reaction(my_faction, &faction.name, &raws::RAWS.lock().unwrap()),
                ent,
            ));
        }
    }
}
