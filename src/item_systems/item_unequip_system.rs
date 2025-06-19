use std::collections::HashSet;

use crate::{
    components::{Equipped, GrantsStatus, InContainer, StatusEffect, WantsToUnequipItems},
    gamelog::DEBUGLOG,
    State,
};

pub fn item_unequip_system(state: &mut State) {
    let mut items_to_unequip = Vec::new();
    let mut effects_to_despawn = Vec::new();

    for (ent, wants_unequip) in state.world.query_mut::<&WantsToUnequipItems>() {
        items_to_unequip.push((ent, wants_unequip.item_entities.clone()));
    }

    for (ent, items) in items_to_unequip.iter() {
        for item in items.iter() {
            match state.world.get::<&GrantsStatus>(*item) {
                Err(_) => {}
                Ok(_) => effects_to_despawn.push((*item, *ent)),
            }

            let _ = state.world.insert_one(*item, InContainer { owner: *ent });
            let _ = state.world.remove_one::<Equipped>(*item);
        }

        let _ = state.world.remove_one::<WantsToUnequipItems>(*ent);
    }

    for (_item, ent) in effects_to_despawn.iter() {
        let status_query = state
            .world
            .query_mut::<&StatusEffect>()
            .into_iter()
            .filter(|f| f.1.target == *ent)
            .map(|f| f.0)
            .collect::<HashSet<_>>();

        for status in status_query.iter() {
            state.world.despawn(*status).unwrap_or_else(|_| {
                DEBUGLOG.add_log(String::from("Couldn't despawn status effect"))
            });
        }
    }
}
