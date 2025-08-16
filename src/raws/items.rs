use new_egui_macroquad::egui::ahash::HashMap;
use serde::Deserialize;

use crate::raws::Renderable;

#[derive(Deserialize, Debug)]
pub struct Item {
    pub name: String,
    pub renderable: Option<Renderable>,
}

pub struct Equippable {
    pub slot: String,
    pub on_equip: HashMap<String, String>,
}

/// Effect types are OnHit, OnDamageRecieved, OnEquip, OnDeath, OnUse
pub struct Effect {
    pub effect_type: String,
}
