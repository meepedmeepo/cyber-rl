use std::{borrow::Borrow, clone, collections::HashMap, hash::Hash, string};

use bracket_lib::{color::RGB, random::DiceType};
use hecs::{BuiltEntity, Entity, EntityBuilder};

use super::{Consumable, Mob, MobStats, Raws, Reaction, Renderable};
use crate::{ai::Energy, components, effects::{Particle, ParticleAnimation, ParticleBurst, ParticleLine}, randomtable::RandomTable, statistics::{self, Pools, StatPool}, AoE, Attribute, BlocksTiles, DamageEffect, EquipmentDirty, EquipmentSlot, Equippable, Faction, FoV, GivesFood, HealingEffect, Hidden, Monster, Name, Naturals, Position, RangedTargetting, RangedWeapon, SingleActivation, Trigger, TriggerOnEnter, Usable, WeaponStat};

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
    prop_index : HashMap<String, usize>,
    faction_index : HashMap<String, HashMap<String, Reaction>>

}

impl RawMaster
{
pub fn empty() -> RawMaster
{
    RawMaster
    {
        raws : Raws{items : Vec::new(), mobs: Vec::new(), spawn_table: Vec::new(), props: Vec::new(), faction_table : Vec::new()},
        item_index : HashMap::new(),
        mob_index: HashMap::new(),
        prop_index : HashMap::new(),
        faction_index : HashMap::new(),
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

    for (k, prop) in self.raws.props.iter().enumerate()
    {
        self.prop_index.insert(prop.name.clone(), k);
    }

    for faction in self.raws.faction_table.iter()
    {
        let mut reactions : HashMap<String, Reaction> = HashMap::new();
        for other in faction.responses.iter()
        {
            reactions.insert(other.0.clone(), match other.1.as_str() 
            {
                "ignore" => Reaction::Ignore,
                "flee" => Reaction::Flee,
                _ => Reaction::Attack
            });
        }
        self.faction_index.insert(faction.name.clone(), reactions);
    }

}

fn add_renderable_comp(entity_builder: EntityBuilder, renderable : &Renderable) -> EntityBuilder
{
    let mut  eb = entity_builder;
    eb.add(components::Renderable
        {
            glyph: renderable.glyph.chars().next().unwrap(),
            fg: bracket_lib::color::RGB::from_hex(&renderable.fg).expect("Invalid RBG"),
            bg: bracket_lib::color::RGB::from_hex(&renderable.bg).expect(format!("Invalid RGB {}",renderable.glyph.clone()).as_str()),
            order: renderable.order,
        });

    eb
}

fn add_effects_comps(entity_builder: EntityBuilder, effects: HashMap<String, String>) -> EntityBuilder
{
    let mut eb = entity_builder;

    for effect in effects.iter()
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
                "food"=>{eb.add(GivesFood{amount: effect.1.parse::<i32>().unwrap()});}
                "particle" => 
                {
                    let particle = RawMaster::parse_particle_string(effect.1.clone());
                    eb.add(ParticleBurst{particle});
                }
                "particleline" => {eb.add(ParticleLine{particle: RawMaster::parse_particle_string(effect.1.clone())});}
                _ =>{bracket_lib::terminal::console::log
                    (format!("Warning: effect {} not implemented.", effect_name));}
            }

                
        }

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

pub fn get_prop_name_list(&self)-> Vec<String>
{
    self.prop_index.keys().map(|key| key.clone()).collect()
}

pub fn parse_particle_string(particle_string : String) -> Particle
{
    let parts = particle_string.split(';').collect::<Vec<_>>();
    
    let glyph = parts[0].parse::<char>().expect("not valid char for particle");
    let fg =  RGB::from_hex(parts[1]).expect("not valid hex rgb for particle fg");
    let bg = RGB::from_hex(parts[2]).expect("not valid hex rgb for particle bg");
    let lifetime = parts[3].parse::<f32>().expect("not valid f32 for particle lifetime");

    Particle{fg,glyph,bg,lifetime}
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
        to_hit_bonus: weapon.to_hit_bonus, dmg_bonus : dmg_die.bonus, num_dmg_dice: dmg_die.n_dice}
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
    let level = stats.level.unwrap_or(1);
    eb.add(statistics::BaseStatistics {

        strength : Attribute::new(str),
        dexterity : Attribute::new(dex),
        toughness : Attribute::new(toughness),
        intelligence : Attribute::new(intelligence),
        mental_fortitude : Attribute::new(mental),
    });

    eb.add(Pools{hitpoints: StatPool::new(stats.max_hp),
        exp : 0, level : level, armour_class: Attribute::new(ac), hit_die: DiceType::new(1, 6, 0)});

    eb
}

pub fn spawn_named_mob<'a>(raws : &'a RawMaster, new_entity : hecs::EntityBuilder,key : &str, pos : SpawnType)
-> (Option<Box<EntityBuilder>>, Vec<String>)
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



        let mut equip_list : Vec<String> = Vec::new();
        if let Some(equipment) = &mob_template.equipment
        {
            equip_list = equipment.clone();

            eb.add(EquipmentDirty{});
        }

        if let Some(faction) = &mob_template.faction
        {
            eb.add(Faction{name : faction.clone()});
        }else
        {
            eb.add(Faction{name : "Mindless".to_string()});    
        }

        

        return (Some((Box::new(eb))), equip_list);
    }


    (None, Vec::new())
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

        if let Some(wearable) = &item_template.wearable
        {
            eb.add(components::Wearable{ac_bonus: wearable.ac_bonus});
        }

        if let Some(range) = &item_template.ranged
        {
            eb.add(RangedWeapon{range: *range});
        }

        if let Some(consumable) = &item_template.consumable
        {
            eb.add(components::Consumable {});
            eb.add(Usable{});

            let effects = consumable.effects.clone();

            eb = RawMaster::add_effects_comps(eb, effects);
            
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
                        "ranged" => {slot = EquipmentSlot::Ranged},
                        "quiver" => {slot = EquipmentSlot::Quiver}
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

        if let Some(proj) = &item_template.rangedprojectile
        {
            eb.add(ParticleAnimation{particle: RawMaster::parse_particle_string(proj.clone())});
        }

        return Some(Box::new(eb) );
    }

    None
}
pub fn spawn_named_prop<'a>(raws : &'a RawMaster, new_entity : hecs::EntityBuilder, key : &str, pos : SpawnType)
    ->Option<Box::<EntityBuilder>>
{
    let mut eb = new_entity;
    if raws.prop_index.contains_key(key)
    {
        let prop_template = &raws.raws.props[raws.prop_index[key]];

        eb.add(Name{name: prop_template.name.clone()});
        
        eb = RawMaster::add_renderable_comp(eb, &prop_template.renderable);

        if let Some(_) = &prop_template.single_activation
        {
            eb.add(SingleActivation{});
        }

        if let Some(_) = &prop_template.entry_trigger
        {
            eb.add(Hidden{});
            eb.add(Trigger{});
            eb.add(TriggerOnEnter{});
        }

        let effects = prop_template.consumable.effects.clone();
        eb = RawMaster::add_effects_comps(eb, effects);

        match pos
        {
            SpawnType::AtPosition { x, y } => {eb.add(Position{x: x, y: y});}
            _ => {}
        }



        return Some(Box::new(eb));
    }

    None{}
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


pub fn faction_reaction(my_faction: &str, their_faction: &str, raws: &RawMaster) -> Reaction
{
    if raws.faction_index.contains_key(my_faction) 
    {
        let mf = &raws.faction_index[my_faction];
        if mf.contains_key(their_faction)
        {
            return mf[their_faction];
        }
        else if mf.contains_key("Default")
        {
            return mf["Default"];
        }
        else 
        {
            return Reaction::Ignore;
        }
    }

    Reaction::Ignore
}