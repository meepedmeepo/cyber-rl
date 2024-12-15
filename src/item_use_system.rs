use std::cmp::min;
use crate::damage_system::DamageSystem;
use crate::{AoE, DamageEffect, HealingEffect,   Name, Position, State, Statistics, WantsToUseItem};
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


        //gets targets
        match ents.4
        {
            Some(target_point) =>
            {
                match item_info[index].5
                {
                    Some(aoe) =>
                    {
                        //panic!("AoE not implemented");

                        let fov = bracket_lib::prelude::field_of_view(target_point, aoe.radius, &state.map);

                        targets.push(Vec::new());

                        for point in fov.iter()
                        {
                            for i in state.map.get_mob_entities_at_position(state, *point)
                            {
                                targets[index].push(i);
                            }
                        }
                    }

                    //Single targeted
                    None => 
                    {
                       targets.push( Vec::new());
                       
                       for i in state.map.get_mob_entities_at_position(state, target_point).iter()
                       {
                            targets[index].push(*i);
                       }
                       
                    }
                }
            }

            //Self targeted
            None =>
            { 
                targets.push(Vec::new());
                targets[index].push(ents.0);
            }
        }
        
       
        //Removes wants to use item tag
        state.world.remove_one::<WantsToUseItem>(ents.0)
                    .expect("Can't find entity to remove WantsToUseItem component");

        //Applies healing effects
        match item_info[index].2
        {
            Some(healing) => 
        {
            for target in targets[index].iter()
            {
                let (stats, name) = 
                state.world.query_one_mut::<(&mut Statistics,&Name)>(*target)
                .expect("Couldn't find stats for target to heal!");
                
                stats.hp = min(stats.hp + healing.healing_amount, stats.max_hp);

                if item_info[index].4.is_none()
                {
                    state.game_log.add_log(format!("{} used {} and healed for {} hp!"
                        ,name.name.clone(), item_info[index].0, healing.healing_amount));

                    bracket_lib::terminal::console::log(format!("{} used {} and healed for {} hp!"
                        ,name.name.clone(), item_info[index].0, healing.healing_amount));
                }
                else 
                { 
                    state.game_log.add_log(format!("{} used {} on {}, and healed for them for {} hp!"
                        ,ents.3.clone(), item_info[index].0, name.name.clone() ,healing.healing_amount));

                    bracket_lib::terminal::console::log(format!("{} used {} on {}, and healed for them for {} hp!"
                        ,ents.3.clone(), item_info[index].0, name.name.clone() ,healing.healing_amount));
                }
            }
        }
        None => {}
        }

        //Applies damage effects
        match item_info[index].3
        {
            Some(dmg) => 
            {
                for target in targets[index].iter()
                {
                    let  name = state.world.query_one_mut::<(&Name)>(*target).expect("Couldn't find entity to mark for damage").name.clone();
                    match item_info[index].4
                    {
                        Some(_) => 
                        {
                            state.game_log.add_log(format!("  on {}!", name));
                            state.game_log.add_log(format!("{} uses {} ",ents.3, item_info[index].0.clone(),));
                        }

                        None => 
                        {
                            state.game_log.add_log(format!("  on themselves!"));
                            state.game_log.add_log(format!("{} uses {}\n", name, item_info[index].0.clone()));
                        }
                    }
                    
                    DamageSystem::mark_for_damage(state, *target, dmg.damage_amount);
                }
            }

            None => {}
        }

    
    
     // Removing item if it was a consumable
     match item_info[index].1
     {
         Some(_consumable) =>
          {
            state.world.despawn(ents.1).expect("Couldn't despawn consumable item from inventory!");
          }

         None => {}
     }


     //end of loop for run function in theory
     index+=1;
    



}}