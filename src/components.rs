use bracket_lib::prelude::Point;
use hecs::Entity;


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
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ItemContainer
{
    pub items : Vec<Entity>,
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