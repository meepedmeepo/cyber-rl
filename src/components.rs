use bracket_lib::prelude::Point;
use hecs::Entity;

#[derive(Clone, Copy)]
pub enum WeaponStat
{
    Strength,
    Dexterity
}

pub struct Naturals
{
    pub weapons : Vec<Weapon>
}

#[derive(Clone, Copy)]
pub struct Weapon
{
   pub uses_statistic : WeaponStat,

   pub damage_die : i32,
   pub to_hit_bonus : i32,
   pub dmg_bonus: i32,
}

pub struct RangedWeapon
{
    pub range: i32
}
pub struct Usable
{}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WantsToEquipItem
{
    pub item : Entity,
    pub slot : EquipmentSlot,
}
#[derive(Clone, Copy, PartialEq,Eq, PartialOrd, Ord)]
pub struct Attribute
{
   pub base : i32,
   pub bonuses : i32,
   pub total : i32,
   pub dirty : bool,
}

impl Attribute
{
    pub fn new(base : i32) -> Attribute
    {
        Attribute
        {
            base : base,
            bonuses: 0,
            total: base,
            dirty: true,
        }
    }

    pub fn get_modifier(&self) -> i32
    {
        (self.total - 10) / 2
    }
}

#[derive(Clone, Copy, PartialEq,Eq, PartialOrd, Ord)]
pub struct CombatStats
{
    pub power : Attribute,
    pub defence : Attribute
}
impl CombatStats
{
    pub fn is_dirty(&self)-> bool
    {
        if self.power.dirty || self.defence.dirty
        {
            return true;
        }
        else 
        {
            false
        }
    }

    pub fn new(power : i32, defence : i32) -> CombatStats
    {
        CombatStats
        {
            power : Attribute::new(power),
            defence : Attribute::new(defence),
        }
    }
}
#[derive(Clone, Copy, PartialEq,Eq, PartialOrd, Ord)]
#[allow(dead_code)]
pub enum EquipmentSlot
{
    Head,
    Body,
    Legs,
    Boots,
    Hands,
    MainHand,
    OffHand,
    Ranged
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Equipped
{
    pub owner : Entity,
    pub slot : EquipmentSlot,
}

#[derive(Clone, Copy, PartialEq,Eq, PartialOrd, Ord)]
pub struct Equippable
{
    pub slot : EquipmentSlot,
    pub power_bonus : i32,
    pub defence_bonus : i32,
}



#[derive(Clone, Copy, PartialEq,Eq, PartialOrd, Ord)]
pub struct InContainer
{
    pub owner : Entity,
}

pub struct Renderable
{
   pub glyph : char,
   pub fg : bracket_lib::color::RGB,
    pub bg : bracket_lib::color::RGB,
    pub order : i32,
}

#[derive(Clone)]
pub struct Name
{
    pub name : String,
}

impl Renderable
{
   pub fn new(glyph: char,fg : bracket_lib::color::RGB, bg: bracket_lib::color::RGB,order : i32) -> Renderable
    {
        Renderable
        {
            glyph,
            fg,
            bg,
            order
        }
    }
}
#[derive(Clone, Copy, PartialEq,Eq, PartialOrd, Ord)]
pub struct PowerBonus
{
    bonus : i32
}
#[derive(Clone, Copy, PartialEq,Eq, PartialOrd, Ord)]
pub struct DefenceBonus
{
    bonus : i32
}
#[derive(Clone, Copy, PartialEq,Eq, PartialOrd, Ord)]
pub struct AoE
{
   pub radius: i32,
}

#[derive(Clone, Copy, PartialEq,Eq, PartialOrd, Ord)]
pub struct RangedTargetting
{
    pub range : i32,
}

#[derive(Clone, Copy,PartialEq, Eq, PartialOrd, Ord)]
pub struct DamageEffect
{
   pub damage_amount: i32,
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Item
{}
#[derive(Clone, Copy,PartialEq, Eq, PartialOrd, Ord)]
pub struct HealingEffect
{
   pub healing_amount: i32,
}

#[derive(Clone, Copy,PartialEq, Eq, PartialOrd, Ord)]
pub struct WantsToPickupItem
{
    pub item : Entity,
    
}
#[derive(Clone, Copy,PartialEq, Eq, PartialOrd, Ord)]
pub struct Consumable
{}
pub struct Monster
{}
#[derive(Clone, Copy,PartialEq, Eq)]
pub struct WantsToUseItem
{
    pub item : Entity,
    pub target: Option<Point>
}
pub struct BlocksTiles
{}

pub struct Attack
{
   pub target: Entity,
}

/// This is used to mark all the damage that an entity will take that is processed by the damage_system
pub struct TakeDamage
{
    pub damage_to_take : Vec<i32>,
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Statistics
{
    pub max_hp : i32,
    pub hp : i32,
    pub strength: i32,
    pub defence : i32,
}

pub struct FoV
{
    pub visible_tiles: Vec<Point>,
    pub range : i32,
    pub dirty: bool,
}

impl FoV
{
    pub fn new(range:i32) ->FoV
    {
        FoV
        {
            range,
            visible_tiles: Vec::new(),
            dirty : true,
        }
    }

}