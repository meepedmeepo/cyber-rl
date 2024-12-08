use crate::{DamageEffect, HealingEffect, Item, Name, Position, RangedTargetting, Renderable, State};
use crate::components::Consumable;
use bracket_lib::terminal::Point;



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