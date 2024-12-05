use std::cmp::min;

use crate::{HealingEffect, ItemContainer, Name, Position, State, Statistics, WantsToUseItem};




pub fn run(state : &mut State)
{
    let mut entities_to_use_items = Vec::new();
    for (_id, (item_use_tag,_pos,stats,name)) in 
    state.world.query_mut::<(&WantsToUseItem,&Position,&Statistics,&Name)>()
    {
        entities_to_use_items.push((_id, item_use_tag.item, *stats, name.name.clone()));
    }

    //removes used item from container and then removes the WantsToUseItem component from that entity afterwards
    for ents in entities_to_use_items.iter()
    {
        let mut items = state.world.get::<&ItemContainer>(ents.0)
        .expect("Couldn't get item container from entity!")
        .items.clone();
        
        items.remove(items.iter().position(|x| *x == ents.1)
        .expect("Couldn't find item in ItemContainer of entity!"));

        state.world.insert_one(ents.0, ItemContainer {items: items})
        .expect("Couldn't find entity to insert item!");

        state.world.remove_one::<WantsToUseItem>(ents.0)
        .expect("Can't find entity to remove WantsToUseItem component");
    }

    for ents in entities_to_use_items.iter_mut()
    {
        let mut is_dirty = false;
        {
        let mut query = 
        state.world.query_one::<(Option<&HealingEffect>,&Name)>
        (ents.1).unwrap();


    let (effect,name) = query.get().unwrap();
    match effect
    {
        Some(healing) => 
        {
            ents.2.hp = min(ents.2.hp + healing.healing_amount, ents.2.max_hp);
            bracket_lib::terminal::console::log(format!("{} used {} and healed for {}!",ents.3,name.name, healing.healing_amount));
            is_dirty = true;
        }
        None => {}
    }
    }
    if is_dirty
    {
        state.world.insert_one(ents.0, ents.2).expect("Couldn't change stats for entity after using item!");
    }
    }



}