use std::{collections::HashMap, fs};
use serde::Deserialize;
mod rawmaster;
pub use rawmaster::*;
mod spawn_table_structs;
use spawn_table_structs::*;
use std::sync::Mutex;
use crate::lazy_static::LazyStatic;
//makes it safe to use RawMaster as a global static singleton.
lazy_static! {
    pub static ref RAWS : Mutex<RawMaster> = Mutex::new(RawMaster::empty());
}


#[derive(Deserialize, Debug)]
pub struct Raws
{
    pub items : Vec<Item>,
    pub mobs : Vec<Mob>,
    pub props : Vec<Prop>,
    pub spawn_table : Vec<SpawnTableEntry>
}

#[derive(Deserialize, Debug)]
pub struct Item
{
    pub name :String,
    pub renderable : Option<Renderable>,
    pub consumable : Option<Consumable>,
    pub equippable: Option<EquipmentStats>,
    pub weapon: Option <Weapon>,
    pub ranged: Option<i32>,
    pub wearable: Option<Wearable>
}
#[derive(Debug, Deserialize, Clone)]
pub struct Weapon
{
    pub statistic : String,
    pub damage_die : String,
    pub to_hit_bonus : i32
}
#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Wearable
{
    pub ac_bonus : i32,
}
#[derive(Deserialize, Debug)]
pub struct EquipmentStats
{
    pub slot: String,

}
#[derive(Deserialize, Debug)]
pub struct Mob
{
    pub name : String,
    pub renderable : Renderable,
    pub stats : MobStats,
    pub vision_range: i32,
    pub blocks_tiles: bool,
    pub naturals: Option<Vec<Weapon>>,
    pub equipment: Option<Vec<String>>
}

#[derive(Deserialize, Debug)]
pub struct MobStats
{
    pub max_hp : i32,
    pub hp: i32,
    pub strength : Option<i32>,
    pub dexterity : Option<i32>,
    pub toughness : Option<i32>,
    pub intelligence : Option<i32>,
    pub mental : Option<i32>,
    pub natural_ac : Option<i32>,
    pub level : Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct Renderable
{
    pub glyph : String,
    pub fg : String,
    pub bg : String,
    pub order : i32
}

#[derive(Deserialize, Debug)]
pub struct Consumable
{
    pub effects : HashMap<String,String>
}
#[derive(Deserialize, Debug)]
pub struct Prop
{
    pub name : String,
    pub renderable : Renderable,
    pub single_activation : Option<bool>,
    pub entry_trigger : Option<bool>,
    pub consumable : Consumable,
}

pub fn run()
{
    let data = fs::read_to_string(std::path::Path::new("./src/raws/spawns.json"))
        .expect("Unable to read spawns.json");
    println!("{}", data);
    let decoder : Raws = serde_json::from_str(&data).expect("Unable to parse JSON");
    bracket_lib::terminal::console::log(format!("{:?}", decoder));
    RAWS.lock().unwrap().load(decoder);
}