use crate::raws::{SpawnType, RAWS};
use crate::Map;
use crate::{DamageEffect, HealingEffect, Item, Name, Position, RangedTargetting, Renderable, State,raws::RawMaster};
use crate::components::Consumable;
use bracket_lib::prelude::Rect;
use bracket_lib::terminal::Point;

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
        spawn_room(state, *room);
    }
}
pub fn spawn_room(state : &mut State, room : Rect)
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

    let res = state.rng.roll_dice(1, 6);
    
    if res < 3
    {return;}
    else if res > 2 && res < 5
    {
        num_mobs = state.rng.roll_dice(1,4)
    }
    else if res == 5 || res == 6
    {
        num_items = state.rng.roll_dice(1, 2);
    }

    while num_items > 0
    {
        let posO = room.point_set();
        let pos = posO.iter().next().unwrap();
        let num = state.rng.random_slice_index(item_names.as_slice()).unwrap();
        spawn_entity(state, &(&0,&item_names[num]), pos.x, pos.y, EntityType::Item);

        num_items-=1;
    }

    while num_mobs > 0
    {
        let pos_set = room.point_set();

        let mut attempts = 20;
        for pos in pos_set.iter()
        {
            let idx = Map::xy_id(pos.x, pos.y);

            if !state.map.blocked[idx]
            {
                let num = state.rng.random_slice_index(mob_names.as_slice()).unwrap();
                spawn_entity(state, &(&0,&mob_names[num]), pos.x, pos.y, EntityType::Mob);
                num_mobs-=1;
                break;
            }

            attempts -= 1;
            if attempts < 1
            {
                break;
            }
        }
    }


}