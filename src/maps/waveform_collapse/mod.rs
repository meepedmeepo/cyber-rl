mod common;
mod solver;
mod constraints;
use bracket_lib::random::RandomNumberGenerator;
use common::*;
use constraints::{build_patterns, patterns_to_constraints};
use solver::Solver;

use crate::Map;

use super::{BuilderMap, MetaMapBuilder};




pub struct WaveformCollapseBuilder {}

impl MetaMapBuilder for WaveformCollapseBuilder
{
    fn build_map(&mut self, rng: &mut bracket_lib::prelude::RandomNumberGenerator, build_data: &mut super::BuilderMap) 
    {
        self.build(rng, build_data);
    }
}

impl WaveformCollapseBuilder
{
    pub fn new() -> Box<WaveformCollapseBuilder>
    {
        Box::new(WaveformCollapseBuilder {})
    }

    fn build(&mut self, rng : &mut RandomNumberGenerator, build_data : &mut BuilderMap)
    {
        const CHUNK_SIZE : i32 = 5;

        let patterns = build_patterns(&build_data.map, CHUNK_SIZE, true, true);
        let constraints = patterns_to_constraints(patterns, CHUNK_SIZE);
        
        build_data.map = Map::new(build_data.map.depth, build_data.map.map_width, build_data.map.map_height);

        loop 
        {
            let mut solver = Solver::new(constraints.clone(), CHUNK_SIZE, &build_data.map);
            while !solver.iteration(&mut build_data.map, rng) 
            {

            }   

            if solver.possible {break;}//impossible condition - needs to try again!
        }
        build_data.spawn_list.clear();
        
    }
}