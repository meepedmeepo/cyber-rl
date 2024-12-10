use std::collections::HashMap;

use hecs::{BuiltEntity, Entity, EntityBuilder};

use super::{Consumable, Raws};
use crate::{components, DamageEffect, HealingEffect, Name, Position, RangedTargetting};

pub enum SpawnType 
{
    AtPosition { x: i32, y: i32 },
}
pub struct RawMaster
{
    raws : Raws,
    item_index : HashMap<String, usize>

}

impl RawMaster
{
pub fn empty() -> RawMaster
{
    RawMaster
    {
        raws : Raws{items : Vec::new()},
        item_index : HashMap::new(),
    }
}

pub fn load(&mut self, raws : Raws)
{
    self.raws = raws;
    self.item_index = HashMap::new();
    for (i,item) in self.raws.items.iter().enumerate()
    {
        self.item_index.insert(item.name.clone(),i);
    }
}

pub fn spawn_named_item<'a>(raws : &'a RawMaster, new_entity : hecs::EntityBuilder, key : &str, pos : SpawnType)
 ->Option<Box::<EntityBuilder>>
 {
    if raws.item_index.contains_key(key)
    {
        let item_template = &raws.raws.items[raws.item_index[key]];
        let mut eb = new_entity;
        
        //adds position component
        match pos
        {
            SpawnType::AtPosition { x, y }     =>
            {
                eb.add(Position{x: x, y: y});
            }
        }

        //adds renderable component
        if let Some(renderable) = &item_template.renderable
        {
            eb.add(components::Renderable 
                {
                    glyph: renderable.glyph.chars().next().unwrap(),
                    fg: bracket_lib::color::RGB::from_hex(&renderable.fg).expect("Invalid RBG"),
                    bg: bracket_lib::color::RGB::from_hex(&renderable.bg).expect("Invalid RBG"),
                    order: renderable.order 
                });
        }

        eb.add(Name{name: item_template.name.clone()});

        eb.add(components::Item{});

        if let Some(consumable) = &item_template.consumable
        {
            eb.add(components::Consumable {});
            for effect in consumable.effects.iter()
            {
                let effect_name = effect.0.as_str();
                
                match effect_name
                {
                    "provides_healing" =>
                    {
                        eb.add(HealingEffect {healing_amount: effect.1.parse::<i32>().unwrap()});
                    }
                    "ranged" => {eb.add(RangedTargetting{range: effect.1.parse::<i32>().unwrap()});}
                    "damage" => {eb.add(DamageEffect{damage_amount: effect.1.parse::<i32>().unwrap()});}
                    _ =>{bracket_lib::terminal::console::log(format!("Warning: consumable effect {} not implemented.", effect_name));}
                }
            }
            
        }

        return Some(Box::new(eb) );
    }

    None
 }
    
}