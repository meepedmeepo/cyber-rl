use hecs::Entity;

use crate::{
    raws::{self, Reaction},
    utils::get_mobs_at_idx,
    Attack, Faction, Player, Position, State,
};

use super::{apply_energy_cost, InCombat, MyTurn};

pub fn adjacent_ai_system(state: &mut State) {
    let mut turn_done: Vec<Entity> = Vec::new();
    let mut attacks: Vec<(Entity, Entity)> = Vec::new();

    for (ent, (_turn, faction, pos)) in state
        .world
        .query::<(&MyTurn, &Faction, &Position)>()
        .without::<&Player>()
        .iter()
    {
        let mut reactions: Vec<(Entity, raws::Reaction)> = Vec::new();
        let my_faction = &faction.name;
        let idx = state.map.xy_idx(pos.x, pos.y);
        let w = state.map.map_width;
        let h = state.map.map_height;

        //checking possible reactions for each of the 8 cardinal directions
        if pos.x > 0 {
            evaluate(idx - 1, state, my_faction, &mut reactions);
        }
        if pos.x < w - 1 {
            evaluate(idx + 1, state, my_faction, &mut reactions);
        }
        if pos.y > 0 {
            evaluate(idx - w as usize, state, my_faction, &mut reactions);
        }
        if pos.y < h - 1 {
            evaluate(idx + w as usize, state, my_faction, &mut reactions);
        }
        if pos.y > 0 && pos.x > 0 {
            evaluate((idx - w as usize) - 1, state, my_faction, &mut reactions);
        }
        if pos.y > 0 && pos.x < w - 1 {
            evaluate((idx - w as usize) + 1, state, my_faction, &mut reactions);
        }
        if pos.y < h - 1 && pos.x > 0 {
            evaluate((idx + w as usize) - 1, state, my_faction, &mut reactions);
        }
        if pos.y < h - 1 && pos.x < w - 1 {
            evaluate((idx + w as usize) + 1, state, my_faction, &mut reactions);
        }

        let mut done = false;
        for reaction in reactions.iter() {
            if let Reaction::Attack = reaction.1 {
                done = true;
                attacks.push((ent, reaction.0));
            }
        }

        if done == true {
            turn_done.push(ent);
        }
    }
    //todo: change this so that a random target is selected from possible enemies if there are several valid targets for an entity!
    for attack in attacks.iter() {
        let _ = state
            .world
            .insert_one(attack.0, Attack { target: attack.1 });

        let _ = state
            .world
            .insert_one(attack.0, InCombat { target: attack.1 });
    }

    for done in turn_done.iter() {
        let _ = state.world.remove_one::<MyTurn>(*done);

        apply_energy_cost(state, super::ActionType::Attack, *done);
    }
}

fn evaluate(
    idx: usize,
    state: &State,
    my_faction: &str,
    reactions: &mut Vec<(Entity, raws::Reaction)>,
) {
    get_mobs_at_idx(state, idx).iter().for_each(|ent| {
        if let Ok(faction) = state.world.get::<&Faction>(*ent) {
            reactions.push((
                *ent,
                raws::faction_reaction(my_faction, &faction.name, &raws::RAWS.lock().unwrap()),
            ));
        }
    });
}
