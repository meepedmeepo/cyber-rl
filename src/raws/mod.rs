use std::{collections::HashMap, fs};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Raws
{
    pub items : Vec<Item>,
   // pub mobs : Vec<Mob>
}
#[derive(Deserialize, Debug)]
pub struct Item
{
    pub name :String,
    pub renderable : Option<Renderable>,
    pub consumable : Option<Consumable>
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


fn run()
{
let data = fs::read_to_string("./raws/spawns.json").expect("Unable to read spawns.json");
    println!("{}", data);
    let decoder : Raws = serde_json::from_str(&data).expect("Unable to parse JSON");
}