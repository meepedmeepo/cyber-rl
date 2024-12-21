use std::collections::HashSet;

use crate::raws::{get_spawn_table_for_depth, SpawnType, RAWS};
use crate::{Map, TileType};
use crate::{DamageEffect, HealingEffect, Item, Name, Position, RangedTargetting, Renderable, State,raws::RawMaster};
use crate::components::Consumable;
use bracket_lib::prelude::Rect;
use bracket_lib::terminal::Point;

use super::randomtable::RandomTable;


pub const MAXMOBS : i32 = 8;
pub enum EntityType
{
    Item,
    Mob
}

pub fn spawn_healing_item(state : &mut State) 
{
   let mut pos = state.map.rooms[0].center();
    pos += Point{x:1,y:1};

    state.world.spawn((Position::new(pos.x, pos.y),
    Name{name : "Healing Potion".to_string()},
    Renderable::new('¡', bracket_lib::color::RGB::from_f32(1., 0., 1.), bracket_lib::color::RGB::from_f32(0., 0., 0.), 2),
    Item{}, Consumable{}, HealingEffect{healing_amount: 15}));

}

pub fn spawn_damage_item(state : &mut State) 
{
   let mut pos = state.map.rooms[0].center();
    pos += Point{x:0,y:1};

    state.world.spawn((Position::new(pos.x, pos.y),
    Name{name : "Magic Missile".to_string()},
    Renderable::new('%', bracket_lib::color::RGB::from_f32(0.5, 0., 0.5), bracket_lib::color::RGB::from_f32(0., 0., 0.), 2),
    Item{}, Consumable{},RangedTargetting {range: 3}, DamageEffect{damage_amount: 10}));

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
            let  mob_res = 
                RawMaster::spawn_named_mob(&RAWS.lock().unwrap(), hecs::EntityBuilder::new(),
            &spawn.1, SpawnType::AtPosition{ x, y});
            match mob_res
            {
                Some(mut mob) => 
                {
                    state.world.spawn(mob.build());

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
    
}

}

pub fn room_spawns( state : &mut State)
{
    let rooms = state.map.rooms.clone();
    for room in rooms.iter().skip(1)
    {
        spawn_room(state, *room, state.map.depth);
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
        }
        else
        {
            panic!("{} is not a valid item or mob name so can't be spawned", name);    
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