use std::collections::HashMap;

use bracket_lib::{
    prelude::{FontCharType, Point},
    random::DiceType,
};
use hecs::Entity;
use serde::Deserialize;

use crate::statistics::StatType;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MovementType {
    Static,
    RandomWaypoint { path: Option<(Vec<usize>, usize)> },
    Drunk,
}

pub struct AIQuips {
    pub quips: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
pub struct BlocksVisibility {}

#[derive(Debug, Clone, Copy)]
pub struct Door {
    pub open: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Projectile {
    pub damage: DiceType,
}

pub struct Creator {
    pub ent: Entity,
}
#[derive(Debug, Clone)]
pub struct WantsToApproach {
    pub target: i32,
}
#[derive(Debug, Clone)]
pub struct WantsToFlee {
    pub indices: Vec<usize>,
}
#[derive(Clone, PartialEq, Eq, Deserialize, Debug)]
pub struct Faction {
    pub name: String,
}

pub struct Prop {}

pub struct Hidden {}
#[derive(Clone, Copy)]
pub struct Triggered {
    pub entity: Entity,
    pub idx: i32,
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DescendFloors {
    pub num_floors: u32,
}

pub struct TriggerOnEnter {}

pub struct SingleActivation {}

pub struct Trigger {}

pub struct HasMoved {}

pub struct WantsToRest {}

pub struct Password {
    pub controls_target: Entity,
    pub owner: Entity,
}

pub struct PasswordProtected {
    pub difficulty_class: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct GivesFood {
    pub amount: i32,
}

pub struct EquipmentDirty {}

#[derive(Clone, Copy)]
pub enum WeaponStat {
    Strength,
    Dexterity,
}
#[derive(Clone, Copy)]
pub struct Wearable {
    pub ac_bonus: i32,
}
pub struct Naturals {
    pub weapons: Vec<Weapon>,
}

#[derive(Clone, Copy)]
pub struct Weapon {
    pub uses_statistic: WeaponStat,

    pub damage_die: i32,
    pub to_hit_bonus: i32,
    pub dmg_bonus: i32,
    pub num_dmg_dice: i32,
}

pub struct RangedWeapon {
    pub range: i32,

    pub damage: DiceType,
}
pub struct Usable {}

#[derive(Debug)]
///Points towards effect spawner to activate when interacted with
pub struct Interactable;
#[derive(Debug)]
pub struct EffectSpawner {
    pub owner: Entity,
}

#[derive(Debug, Clone)]
pub struct EffectSpawnerPrefab {
    pub prefab: HashMap<String, String>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WantsToEquipItem {
    pub item: Entity,
    pub slot: EquipmentSlot,
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Attribute {
    pub base: i32,
    pub bonuses: i32,
    pub total: i32,
    pub dirty: bool,
}

impl Attribute {
    pub fn new(base: i32) -> Attribute {
        Attribute {
            base: base,
            bonuses: 0,
            total: base,
            dirty: true,
        }
    }

    pub fn get_modifier(&self) -> i32 {
        (self.total - 10) / 2
    }

    pub fn get_modifier_display(&self) -> String {
        let modi = self.get_modifier();

        if modi < 0 {
            format!("{}", modi)
        } else {
            format!("+{}", modi)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
pub enum EquipmentSlot {
    Head,
    Body,
    Hands,
    Legs,
    Boots,
    MainHand,
    OffHand,
    Ranged,
    Quiver,
}

#[derive(Clone, Copy)]
pub struct GrantsAttribute {
    pub target_attribute: StatType,
    pub modifier: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct StatusEffect {
    pub target: Entity,
    pub source: Entity,
}

#[derive(Debug, Clone)]
pub struct GrantsStatus {
    pub effects: HashMap<String, String>,
    pub duration: Option<i32>,
}

#[derive(Debug, Clone, Copy)]
pub struct EffectDuration {
    pub rounds: i32,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Equipped {
    pub owner: Entity,
    pub slot: EquipmentSlot,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Equippable {
    pub slot: EquipmentSlot,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct InContainer {
    pub owner: Entity,
}
#[derive(Debug, Clone)]
pub struct Renderable {
    pub glyph: String,
    pub fg: bracket_lib::color::RGB,
    pub bg: bracket_lib::color::RGB,
    pub order: i32,
}

#[derive(Clone)]
pub struct Name {
    pub name: String,
}

impl Renderable {
    pub fn new(
        glyph: String,
        fg: bracket_lib::color::RGB,
        bg: bracket_lib::color::RGB,
        order: i32,
    ) -> Renderable {
        Renderable {
            glyph,
            fg,
            bg,
            order,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AoE {
    pub radius: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RangedTargetting {
    pub range: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DamageEffect {
    pub damage_amount: i32,
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Item {}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HealingEffect {
    pub healing_amount: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct GrantStat {
    pub stat: StatType,
    pub amount: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct WantsToPickupItem {
    pub item: Entity,
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Consumable {}
pub struct Monster {}
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct WantsToUseItem {
    pub item: Entity,
    pub target: Option<Point>,
}
pub struct BlocksTiles {}

pub struct Attack {
    pub target: Entity,
}

/// This is used to mark all the damage that an entity will take that is processed by the damage_system
pub struct TakeDamage {
    pub damage_to_take: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct FoV {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}

impl FoV {
    pub fn new(range: i32) -> FoV {
        FoV {
            range,
            visible_tiles: Vec::new(),
            dirty: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WantsToUnequipItems {
    pub item_entities: Vec<Entity>,
}
