use bracket_lib::prelude::Point;
use hecs::Entity;

use crate::Position;

pub struct Monster
{}

pub struct BlocksTiles
{}

pub struct Attack
{
    target: Entity,
}

pub struct Statistics
{
    max_hp : i32,
    hp : i32,
    strength: i32,
    defence : i32,

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