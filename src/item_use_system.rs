use std::cmp::min;

use crate::damage_system::DamageSystem;
use crate::{AoE, DamageEffect, HealingEffect, ItemContainer, Map, Name, Position, State, Statistics, WantsToUseItem};
use crate::components::Consumable;

enum TargetType
{
    SelfAimed,
    AoE,
    Single,

}

pub fn run(state : &mut State)
{
    let mut entities_to_use_items = Vec::new();

    let mut item_info = Vec::new();

    let mut targets: Vec<Vec<hecs::Entity>> = Vec::new();

    for (_id, (item_use_tag,_pos,stats,name)) in 
    state.world.query_mut::<(&WantsToUseItem,&Position,&Statistics,&Name)>()
    {
        entities_to_use_items.push((_id, item_use_tag.item, *stats, name.name.clone(),item_use_tag.target, false));
    }

    for ents in entities_to_use_items.iter()
    {
        let query = state.world.query_one_mut::<(&Name, Option<&Consumable>,Option<&HealingEffect>, Option<&DamageEffect>, Option<&AoE>)>(ents.1).expect("couldn't get item properties");
        let (name,consumable,healing,damage, aoe) = query;

        item_info.push((name.name.clone(), consumable.copied(), healing.copied(), damage.copied(),ents.4, aoe.copied()));
        
    }

    //rewrite of main function to all be in one loop


    let mut index = 0;
    for ents in entities_to_use_items.iter_mut()
    {

        //This denotes that the entities stats are dirty and needs the component will be replaced at the end of the loop
        let mut is_dirty = false;


        //gets targets
        match ents.4
        {
            Some(target_point) =>
            {
                match item_info[index].5
                {
                    Some(aoe) =>
                    {
                        panic!("AoE not implemented");
                    }

                    //Single targeted
                    None => 
                    {
                       targets[index] = state.map.get_mob_entities_at_position(state, target_point);
                    }
                }
            }

            //Self targeted
            None =>
            { 
                targets[index] = Vec::new();
                targets[index].push(ents.0);
            }
        }
        
        /// Removing item and wants to use item tags
        match item_info[index].1
        {
            Some(consumable) =>
             {
                let mut items = state.world.get::<&ItemContainer>(ents.0)
                    .expect("Couldn't get item container from entity!")
                    .items.clone();

                items.remove(items.iter().position(|x| *x == ents.1)
                    .expect("Couldn't find item in ItemContainer of entity!"));


                state.world.insert_one(ents.0, ItemContainer {items: items})
                    .expect("Couldn't find entity to insert item!");

                

             }

            None => {}
        }
        
        state.world.remove_one::<WantsToUseItem>(ents.0)
                    .expect("Can't find entity to remove WantsToUseItem component");

        //Applies healing effects
        match item_info[index].2
        {
            Some(healing) => 
        {
            for target in targets[index].iter()
            {
                ents.2.hp = min(ents.2.hp + healing.healing_amount, ents.2.max_hp);

                state.game_log.add_log(format!("{} used {} and healed for {}!"
                ,ents.3, item_info[index].0, healing.healing_amount));

                bracket_lib::terminal::console::log(format!("{} used {} and healed for {}!"
                ,ents.3, item_info[index].0, healing.healing_amount));

                is_dirty = true;
            }
        }
        None => {}
        }



    //end of loop for run function in theory
        index+=1;
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

            state.game_log.add_log(format!("{} used {} and healed for {}!"
            ,ents.3,name.name, healing.healing_amount));

            bracket_lib::terminal::console::log(format!("{} used {} and healed for {}!"
            ,ents.3,name.name, healing.healing_amount));

            is_dirty = true;
        }
        None => {}
    }
    std::mem::drop(query);

    //let (damage,name) =
     //state.world.get::<(&DamageEffect, &Name)>(ents.1);
    let mut query = state.world.query_one::<(Option<&DamageEffect>, &Name)>(ents.1).unwrap();
    let (damage,name) = query.get().unwrap();
     match damage
     {
        Some(ref dmg) =>
        {
            match ents.4
            {
                Some(target) =>
                {
                    
                    let dmg_num = dmg.damage_amount;
                    
                    state.game_log.add_log(format!("{} uses {}",ents.3,name.name));

                    let mut ents_to_dmg = Vec::new();
                    for ent in state.map.tile_contents[Map::xy_id(target.x, target.y)].iter()
                    {
                        if state.world.get::<&Statistics>(*ent).is_ok()
                        {
                            ents_to_dmg.push((*ent,dmg_num));
                        }
                    }
                    std::mem::drop(query);
                    for ent in ents_to_dmg
                    {
                        DamageSystem::mark_for_damage(state, ent.0, ent.1);
                    }
                   // 
                }
                None => 
                {
                    let dmg_num = dmg.damage_amount;
                    state.game_log.add_log(format!("{} uses {}",ents.3,name.name));
                    std::mem::drop(query);
                    DamageSystem::mark_for_damage(state, ents.0, dmg_num);
                }
            }
        }
        None => {}
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