use std::{collections::HashMap, fs};
use serde::Deserialize;
mod rawmaster;
pub use rawmaster::*;
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
    pub mobs : Vec<Mob>
}

#[derive(Deserialize, Debug)]
pub struct Item
{
    pub name :String,
    pub renderable : Option<Renderable>,
    pub consumable : Option<Consumable>,
    pub equippable: Option<EquipmentStats>,
}
#[derive(Deserialize, Debug)]
pub struct EquipmentStats
{
    pub slot: String,
    pub power: i32,
    pub defence: i32
}
#[derive(Deserialize, Debug)]
pub struct Mob
{
    pub name : String,
    pub renderable : Renderable,
    pub stats : MobStats,
    pub vision_range: i32,
    pub blocks_tiles: bool,
}

#[derive(Deserialize, Debug)]
pub struct MobStats
{
    pub max_hp : i32,
    pub hp: i32,
    pub power: i32,
    pub defence: i32,
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

pub fn run()
{
let data = fs::read_to_string(std::path::Path::new("./src/raws/spawns.json")).expect("Unable to read spawns.json");
    println!("{}", data);
    let decoder : Raws = serde_json::from_str(&data).expect("Unable to parse JSON");
    bracket_lib::terminal::console::log(format!("{:?}", decoder));
    RAWS.lock().unwrap().load(decoder);
}