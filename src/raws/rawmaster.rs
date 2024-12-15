use std::collections::HashMap;

use hecs::{BuiltEntity, Entity, EntityBuilder};

use super::{Consumable, Mob, MobStats, Raws, Renderable};
use crate::{components, AoE, BlocksTiles, DamageEffect, FoV, HealingEffect, Monster, Name, Position, RangedTargetting};

pub enum SpawnType 
{
    AtPosition { x: i32, y: i32 },
}
pub struct RawMaster
{
    raws : Raws,
    item_index : HashMap<String, usize>,
    mob_index : HashMap<String, usize>,

}

impl RawMaster
{
pub fn empty() -> RawMaster
{
    RawMaster
    {
        raws : Raws{items : Vec::new(), mobs: Vec::new(),},
        item_index : HashMap::new(),
        mob_index: HashMap::new(),
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

    for (j, mob) in self.raws.mobs.iter().enumerate()
    {
        self.mob_index.insert(mob.name.clone(), j);
    }
}

fn add_renderable_comp(entity_builder: EntityBuilder, renderable : &Renderable) -> EntityBuilder
{
    let mut  eb = entity_builder;
    eb.add(components::Renderable
        {
            glyph: renderable.glyph.chars().next().unwrap(),
            fg: bracket_lib::color::RGB::from_hex(&renderable.fg).expect("Invalid RBG"),
            bg: bracket_lib::color::RGB::from_hex(&renderable.bg).expect("Invalid RBG"),
            order: renderable.order,
        });

    eb
}

pub fn get_item_name_list(&self)-> Vec<String>
{
    self.item_index.keys().map(|key| key.clone()).collect()
}

pub fn get_mob_name_list(&self)-> Vec<String>
{
    self.mob_index.keys().map(|key| key.clone()).collect()
}

fn add_position_comp(entity_builder: EntityBuilder, x : i32, y: i32) -> EntityBuilder
{
    let mut eb = entity_builder;
    
    eb.add(Position{x: x, y: y});

    eb
}

fn add_monster_stats_comp(new_entity: EntityBuilder, stats: &MobStats) -> EntityBuilder
{
    let mut eb = new_entity;

    eb.add(components::Statistics
    {
        max_hp: stats.max_hp,
        hp: stats.hp,
        strength: stats.power,
        defence: stats.defence

    });
    
    eb
}

pub fn spawn_named_mob<'a>(raws : &'a RawMaster, new_entity : hecs::EntityBuilder,key : &str, pos : SpawnType)
-> Option<Box<EntityBuilder>>
{
    if raws.mob_index.contains_key(key)
    {
        let mut eb = new_entity;

        let mob_template = &raws.raws.mobs[raws.mob_index[key]];

        let renderable = &mob_template.renderable;

        eb = RawMaster::add_renderable_comp(eb, renderable);

        match pos
        {
            SpawnType::AtPosition { x, y } =>
            {
                eb = RawMaster::add_position_comp(eb, x, y);
            }
        }

        let stats = &mob_template.stats;

        eb = RawMaster::add_monster_stats_comp(eb, stats);

        if mob_template.blocks_tiles
        {
            eb.add(BlocksTiles{});
        }

        eb.add(Monster{});

        eb.add(FoV::new(mob_template.vision_range));

        eb.add(Name{name: mob_template.name.clone()});

        return Some(Box::new(eb));
    }


    None
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
        
            eb = RawMaster::add_renderable_comp(eb,renderable);
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
                    "aoe" => {eb.add(AoE{radius: effect.1.parse::<i32>().unwrap()});}
                    _ =>{bracket_lib::terminal::console::log
                        (format!("Warning: consumable effect {} not implemented.", effect_name));}
                }
            }
            
        }

        return Some(Box::new(eb) );
    }

    None
 }
    
}