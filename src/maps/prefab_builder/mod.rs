use bracket_lib::random::RandomNumberGenerator;

use super::{BuilderMap, InitialMapBuilder, MetaMapBuilder};

mod prefab_levels;
mod prefab_section;


#[derive(PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum PrefabMode
{
    RexLevel {template : &'static str},
    Constant {level : prefab_levels::PrefabLevel},
    Sectional {section : prefab_section::PrefabSection},
    RoomVaults
}


pub struct PrefabBuilder
{
    mode: PrefabMode
}

impl MetaMapBuilder for PrefabBuilder 
{
    fn build_map(&mut self, rng: &mut bracket_lib::prelude::RandomNumberGenerator, build_data: &mut super::BuilderMap) 
    {
        
    }
}

impl InitialMapBuilder for PrefabBuilder
{
    fn build_map(&mut self, rng: &mut bracket_lib::prelude::RandomNumberGenerator, build_data: &mut super::BuilderMap) 
    {
        
    }
}
impl PrefabBuilder
{
    pub fn new() -> Box<PrefabBuilder>
    {
        Box::new(PrefabBuilder { mode: PrefabMode::RoomVaults })
    }

    fn build(&mut self, rng : &mut RandomNumberGenerator, build_data: &mut BuilderMap)
    {
        
    }
}