use core::panic;
use std::collections::HashSet;

use crate::ai::Energy;
use crate::raws::{get_spawn_table_for_depth, SpawnType, RAWS};
use crate::{EquipmentSlot, Equippable, Equipped, InContainer, Map, TileType, Usable};
use crate::{DamageEffect, HealingEffect, Item, Name, Position, RangedTargetting, Renderable, State,raws::RawMaster};
use crate::components::Consumable;
use bracket_lib::prelude::{console, Rect};
use bracket_lib::terminal::Point;
use hecs::Entity;

use super::randomtable::RandomTable;


pub const MAXMOBS : i32 = 6;
pub enum EntityType
{
    Item,
    Mob,
    Prop,
}

/// TODO: add checks for if there is already an item equipped in that slot
pub fn spawn_item_equipped(state : &mut State, item_name: &String, target: Entity)
{

    let mut item_builder = 
        RawMaster::spawn_named_item(&RAWS.lock().unwrap(), hecs::EntityBuilder::new(),
        &item_name, SpawnType::Equipped { target });

    let mut slot : Option<EquipmentSlot> = None;
    match item_builder
    {
        Some(mut build_box) => 
        {
            let query = build_box.get::<&Equippable>();
            match query
            {
                Some(equippable) =>
                {
                    slot = Some(equippable.slot);

                    build_box.add(Equipped {owner: target,
                        slot: slot.expect("Couldn't get slot")});

                    state.world.spawn(build_box.build());
                }

                None =>
                {
                    panic!("Can't spawn and equip item {} as it isn't equippable", build_box.get::<&Name>()
                        .expect("Can't get item name!").name)
                }
            }
        }

        None => 
        {
            panic!("No entity builder found!");
        }
    }
}

pub fn spawn_item_in_backpack(state : &mut State, item_name: &String, owner: Entity)
{
    let mut item_builder =
        RawMaster::spawn_named_item( &RAWS.lock().unwrap(), hecs::EntityBuilder::new(),
        &item_name, SpawnType::InBackpack);

    match item_builder
    {
        Some(mut builder) => 
        {
            builder.add(InContainer{owner});
            let ent = state.world.spawn(builder.build());
        }
        None => 
        {
            console::log(format!("Could spawn {} in backpack as no item with that name exists!",
            item_name))
        }
    }
}

pub fn spawn_entity(state : &mut State, spawn: &(&usize,&String),x:i32,y:i32, ent_type : EntityType)
{
    match ent_type
    {
        EntityType::Item =>
        {
            let  item_res = 
                RawMaster::spawn_named_item(&RAWS.lock().unwrap(), hecs::EntityBuilder::new(),
            &spawn.1, SpawnType::AtPosition{ x, y});
            match item_res
            {
                Some(mut item) => 
                {
                    state.world.spawn(item.build());
                }

                None => 
                {
                    bracket_lib::terminal::console::log(
                    format!("Can't find item entity named {}",&spawn.1));
                }     
        }
        }

        EntityType::Mob =>
        {
            let  (mob_res,equip_list) = 
                RawMaster::spawn_named_mob(&RAWS.lock().unwrap(), hecs::EntityBuilder::new(),
            &spawn.1, SpawnType::AtPosition{ x, y});
            match mob_res
            {
                Some(mut mob) => 
                {
                    //gives random energy so not every mob processes on same tick!
                    mob.add(Energy{value: state.rng.range(-120, 71)});

                    let mob_ent = state.world.spawn(mob.build());
                    for eq in equip_list.iter()
                    {
                        spawn_item_equipped(state, eq, mob_ent);
                    }
                    let idx = Map::xy_id(x, y);
                    state.map.blocked[idx] = true;
                }

                None => 
                {
                    bracket_lib::terminal::console::log(
                    format!("Can't find mob entity named {}",&spawn.1));
                } 
            }
        }

        EntityType::Prop => 
        {
            let prop_res = RawMaster::spawn_named_prop(&RAWS.lock().unwrap(),
                hecs::EntityBuilder::new(), &spawn.1, SpawnType::AtPosition { x, y });
            
            match prop_res
            {
                Some(mut prop) =>
                {
                    state.world.spawn(prop.build());
                }

                None =>
                {
                    bracket_lib::terminal::console::log(
                        format!("Can't find prop entity named {}",&spawn.1));
                }
            }


        }
    
}

}

fn room_table(state : &mut State) -> RandomTable
{
    get_spawn_table_for_depth(&RAWS.lock().unwrap(), state.map.depth)
}

pub fn spawn_room(state : &mut State, room : Rect, depth :i32)
{
    //let mob_names = &RAWS.lock().unwrap().get_mob_name_list();

    let mobguard = RAWS.lock().unwrap();
    let mob_names = mobguard.get_mob_name_list();
    std::mem::drop(mobguard);
    let itemguard =  RAWS.lock().unwrap();
    let item_names =itemguard.get_item_name_list();
    std::mem::drop(itemguard);

    let prop_names = RAWS.lock().unwrap().get_prop_name_list();

    
    let mut num_mobs = 0;
    let mut num_items = 0;
    let mut ent_type  = EntityType::Mob;

    let table = room_table(state);
    
    let mut attempts = 20;

    let mut num_spawns = state.rng.range(0, MAXMOBS+1);

    let mut spawn_points : HashSet<usize> = HashSet::new();

    
    while attempts > 0 && num_spawns > 0
    {
        let name = table.roll(&mut state.rng);
        
        if mob_names.contains(&name)
        {
            ent_type = EntityType::Mob;
        } else if item_names.contains(&name)
        {
            ent_type = EntityType::Item;
        }else if prop_names.contains(&name)
        {
            ent_type = EntityType::Prop;
        }
        else
        {
            panic!("{} is not a valid item, mob or prop name so can't be spawned", name);    
        }

        let pos_set = room.point_set();
        let point = pos_set.iter().next().unwrap();
        let pos = Map::xy_id(point.x, point.y);
        if !spawn_points.contains(&pos) && state.map.map[pos] != TileType::Wall
        {
            spawn_entity(state, &(&0, &name), point.x, point.y, ent_type);
            spawn_points.insert(pos);
            num_spawns -= 1;
        }
        else 
        {
            {
                attempts -= 1;
            }
        }
    }

}