use std::iter;

use bracket_lib::prelude::console;

use crate::{
    components::{EffectDuration, GrantsStatus, StatusEffect},
    effects::{add_effect, EffectType, Targets},
    EquipmentDirty, Equipped, InContainer, Item, Name, State, WantsToEquipItem,
};

//todo: seperate out unequipping into a different system, and remove any status granted by an item that is unequipped.
pub fn run(state: &mut State) {
    let mut entities_to_equip_items = Vec::new();

    for (ent, (equip, name)) in state.world.query::<(&WantsToEquipItem, &Name)>().iter() {
        entities_to_equip_items.push((
            ent,
            equip.clone(),
            name.name.clone(),
            "".to_string(),
            false,
        ));
    }

    //finds the name of the item that is going to be equipped so it can be used in the
    //gamelog message about equipping the item
    for (_ent, equip, _ent_name, item_name, grants_status) in entities_to_equip_items.iter_mut() {
        *item_name = state
            .world
            .get::<&Name>(equip.item)
            .expect("Couldn't get item to equip's name!")
            .name
            .clone();

        if state.world.get::<&GrantsStatus>(equip.item).is_ok() {
            *grants_status = true;
        }
    }

    for (index, info) in entities_to_equip_items.iter().enumerate() {
        //Creates list of items that will need to be unequipped as they take up the same slot as the new item that will
        //be equipped.
        let mut items_to_unequip = Vec::new();
        for (id, (equipped, _item)) in state
            .world
            .query::<(&Equipped, &Item)>()
            .iter()
            .filter(|item| item.1 .0.owner == info.0 && item.1 .0.slot == info.1.slot)
        {
            items_to_unequip.push((id, equipped.owner));
        }

        //Unequips items and adds them back to inventory
        for (item, owner) in items_to_unequip.iter() {
            state
                .world
                .remove_one::<Equipped>(*item)
                .expect("Couldn't remove Equipped component from item to be unequipped");

            state
                .world
                .insert_one(*item, InContainer { owner: *owner })
                .expect("Couldn't add InContainer component onto unequipped item!");
        }

        state
            .world
            .insert_one(
                info.1.item,
                Equipped {
                    owner: info.0,
                    slot: info.1.slot,
                },
            )
            .expect("Couldn't equip selected item!");

        state
            .world
            .remove_one::<InContainer>(info.1.item)
            .expect("Couldn't remove InContainer from newly equiped item! ");

        state
            .world
            .remove_one::<WantsToEquipItem>(info.0)
            .expect("Couldn't remove WantsToEquipItem component from entity!");

        state
            .world
            .insert_one(info.0, EquipmentDirty {})
            .expect("Couldn't insert EquipmentDirty onto entity!");

        state
            .game_log
            .add_log(format!("{} equipped {}!", info.2, info.3));

        console::log(format!("{} equipped {}!", info.2, info.3));

        //If equipped item grants a status use add_effect to apply that status.
        if info.4 {
            let query = state
                .world
                .query_one_mut::<(&GrantsStatus, Option<&EffectDuration>)>(info.1.item);
            let (effect, duration) = query.unwrap();
            let duration_option = duration.and_then(|f| Some(f.rounds));

            add_effect(
                Some(info.1.item),
                EffectType::StatusEffect {
                    effects: effect.effects.clone(),
                    duration: duration_option,
                },
                Targets::Single { target: info.0 },
            );
        }
    }
}
