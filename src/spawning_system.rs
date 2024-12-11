use crate::raws::{SpawnType, RAWS};
use crate::{DamageEffect, HealingEffect, Item, Name, Position, RangedTargetting, Renderable, State,raws::RawMaster};
use crate::components::Consumable;
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