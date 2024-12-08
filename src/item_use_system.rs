use std::cmp::min;

use crate::damage_system::DamageSystem;
use crate::{DamageEffect, HealingEffect, ItemContainer, Map, Name, Position, State, Statistics, WantsToUseItem};
use crate::components::Consumable;



pub fn run(state : &mut State)
{
    let mut entities_to_use_items = Vec::new();
    for (_id, (item_use_tag,_pos,stats,name)) in 
    state.world.query_mut::<(&WantsToUseItem,&Position,&Statistics,&Name)>()
    {
        entities_to_use_items.push((_id, item_use_tag.item, *stats, name.name.clone(),item_use_tag.target, false));
    }

    //removes used item from container and then removes the WantsToUseItem component from that entity afterwards
    for ents in entities_to_use_items.iter_mut()
    {
        let mut items = state.world.get::<&ItemContainer>(ents.0)
        .expect("Couldn't get item container from entity!")
        .items.clone();
        
        let is_consumable = state.world.get::<&Consumable>(ents.1);
        match is_consumable
        {
            Ok(_) => 
            {
                items.remove(items.iter().position(|x| *x == ents.1)
                .expect("Couldn't find item in ItemContainer of entity!"));

                std::mem::drop(is_consumable);

                state.world.insert_one(ents.0, ItemContainer {items: items})
                .expect("Couldn't find entity to insert item!");

                state.world.remove_one::<WantsToUseItem>(ents.0)
                .expect("Can't find entity to remove WantsToUseItem component");
                
                ents.5 = true;
            }
            Err(_) =>
            {
                std::mem::drop(is_consumable);
                state.world.remove_one::<WantsToUseItem>(ents.0)
                .expect("Can't find entity to remove WantsToUseItem component");
            }
        }
        
        

        
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

            bracket_lib::terminal::console::log(format!("{} used {} and healed for {}!"
            ,ents.3,name.name, healing.healing_amount));

            is_dirty = true;
        }
        None => {}
    }
    std::mem::drop(query);

    let damage =
     state.world.get::<&DamageEffect>(ents.1);

     match damage
     {
        Ok(ref dmg) =>
        {
            match ents.4
            {
                Some(target) =>
                {
                    
                    let dmg_num = dmg.damage_amount;
                    std::mem::drop(damage);
    
                    let mut ents_to_dmg = Vec::new();
                    for ent in state.map.tile_contents[Map::xy_id(target.x, target.y)].iter()
                    {
                        if state.world.get::<&Statistics>(*ent).is_ok()
                        {
                            ents_to_dmg.push((*ent,dmg_num));
                        }
                    }
                    for ent in ents_to_dmg
                    {
                        DamageSystem::mark_for_damage(state, ent.0, ent.1);
                    }
                   // 
                }
                None => 
                {
                    let dmg_num = dmg.damage_amount;
                    std::mem::drop(damage);
                    DamageSystem::mark_for_damage(state, ents.0, dmg_num);
                }
            }
        }
        Err(_) => {}
     }
    }

    if is_dirty
    {
        state.world.insert_one(ents.0, ents.2)
        .expect("Couldn't change stats for entity after using item!");
    }
    }
    for ents in entities_to_use_items.iter()
    {
        if ents.5 == true
        {
            state.world.despawn(ents.1).expect("Couldn't despawn item entity!");
        }
    }



}