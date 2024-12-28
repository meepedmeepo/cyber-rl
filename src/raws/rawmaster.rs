use std::{borrow::Borrow, clone, collections::HashMap, string};

use bracket_lib::random::DiceType;
use hecs::{BuiltEntity, Entity, EntityBuilder};

use super::{Consumable, Mob, MobStats, Raws, Renderable};
use crate::{components, randomtable::RandomTable, statistics::{self, Pools, StatPool}, AoE, Attribute, BlocksTiles,  DamageEffect, EquipmentSlot, Equippable, FoV, HealingEffect, Monster, Name, Naturals, Position, RangedTargetting, RangedWeapon, Usable, WeaponStat};

pub enum SpawnType 
{
    AtPosition { x: i32, y: i32 },
    Equipped{target: Entity},
    InBackpack
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
        raws : Raws{items : Vec::new(), mobs: Vec::new(), spawn_table: Vec::new()},
        item_index : HashMap::new(),
        mob_index: HashMap::new()
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

fn parse_weapon_comp(weapon : super::Weapon) -> components::Weapon
{
    let slotname = weapon.statistic.as_str(); 
    let mut dmg_die: DiceType = DiceType::new(0, 0, 0);
    let dmg_die_query = bracket_lib::random::parse_dice_string(&weapon.damage_die);
    match dmg_die_query
    {
        Ok(die) =>
        {
            dmg_die = die;
        }
        Err(e) =>
        {
            panic!("Error: could parse damage dice string correctly! {0}",e);
        }
    }
    let mut stat = WeaponStat::Strength;
    match slotname
    {
        "strength" => {stat = WeaponStat::Strength;}
        "dexterity" => {stat = WeaponStat::Dexterity;}
        _ => {panic!("error that wasn't a valid weapon statistic!");}
    }

    components::Weapon {uses_statistic: stat,damage_die:dmg_die.die_type,
         to_hit_bonus: weapon.to_hit_bonus, dmg_bonus : dmg_die.bonus}
}

fn add_monster_stats_comp(new_entity: EntityBuilder, stats: &MobStats) -> EntityBuilder
{
    let mut eb = new_entity;
    let str = stats.strength.unwrap_or(10);
    let dex = stats.dexterity.unwrap_or(10);
    let toughness = stats.toughness.unwrap_or(10);
    let intelligence = stats.intelligence.unwrap_or(10);
    let mental = stats.mental.unwrap_or(10);
    let ac = stats.natural_ac.unwrap_or(10);

    eb.add(statistics::BaseStatistics {

        strength : Attribute::new(str),
        dexterity : Attribute::new(dex),
        toughness : Attribute::new(toughness),
        intelligence : Attribute::new(intelligence),
        mental_fortitude : Attribute::new(mental),
    });

    eb.add(Pools{hitpoints: StatPool::new(stats.max_hp),
        exp : 0, level : 1, armour_class: ac});

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
            _ =>{}
        }

        let stats = &mob_template.stats;

        eb = RawMaster::add_monster_stats_comp(eb, stats);

        let mut weps = Vec::new();
        let naturals = &mob_template.naturals.clone().unwrap_or(Vec::new());
        for wep in naturals
        {
            weps.push(Self::parse_weapon_comp(wep.clone()));
        }

        eb.add(Naturals{weapons : weps.clone()});

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
            _ => {}
        }

        //adds renderable component
        if let Some(renderable) = &item_template.renderable
        {
        
            eb = RawMaster::add_renderable_comp(eb,renderable);
        }

        eb.add(Name{name: item_template.name.clone()});

        eb.add(components::Item{});

        if let Some(range) = &item_template.ranged
        {
            eb.add(RangedWeapon{range: *range});
        }

        if let Some(consumable) = &item_template.consumable
        {
            eb.add(components::Consumable {});
            eb.add(Usable{});
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
        if let Some(equipment) = &item_template.equippable
        {
                    let slot: EquipmentSlot;
                    let slotname = equipment.slot.as_str();
                    match slotname
                    {
                        "head" => {slot = EquipmentSlot::Head},
                        "hands" => {slot = EquipmentSlot::Hands},
                        "boots" => {slot = EquipmentSlot::Boots},
                        "body" => {slot = EquipmentSlot::Body},
                        "legs" => {slot = EquipmentSlot::Legs},
                        "mainhand" => {slot = EquipmentSlot::MainHand}
                        "offhand" => {slot = EquipmentSlot::OffHand},
                        "ranged" => {slot = EquipmentSlot::Ranged}
                        _ => {panic!("Equipment slot incorrect in json!");}
                    }
                    eb.add(Equippable
                        {
                            slot: slot,
                        });

        }
        
        if let Some(weapon) = &item_template.weapon
        {
            eb.add(Self::parse_weapon_comp(weapon.clone()));
        }

        return Some(Box::new(eb) );
    }

    None
 }
    
}

pub fn get_spawn_table_for_depth(raws : &RawMaster, depth : i32) -> RandomTable
{
    use super::SpawnTableEntry;


    let available_options : Vec<&SpawnTableEntry> = raws.raws.spawn_table
        .iter()
        .filter(|a| depth >= a.min_depth && depth <= a.max_depth)
        .collect();

    let mut rt = RandomTable::new();
    for i in available_options.iter()
    {
        let mut weight = i.weight;
        if i.add_map_depth_to_weight.is_some()
        {
            weight += depth;
        }
        rt = rt.add(i.name.clone(), weight)
    }

    rt
}
