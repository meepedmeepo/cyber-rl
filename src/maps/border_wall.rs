use bracket_lib::random::RandomNumberGenerator;

use super::{MetaMapBuilder, TileType};




pub struct BorderWall{}

impl MetaMapBuilder for BorderWall
{
    fn build_map(&mut self, rng: &mut bracket_lib::prelude::RandomNumberGenerator, build_data: &mut super::BuilderMap) 
    {
        self.build(rng, build_data);
    }
}

impl BorderWall
{
    pub fn new() -> Box<BorderWall>
    {
        Box::new(BorderWall {})
    }

    fn build(&mut self, rng : &mut RandomNumberGenerator, build_data : &mut super::BuilderMap)
    {
        let w = build_data.map.map_width;
        let h = build_data.map.map_height;
        for y in 0 .. h
        {
            for x in 0 .. w
            {
                if x == 1 || y == 1 || x == w-2 || y == h-2
                {
                    let idx = build_data.map.xy_idx(x, y);
                    build_data.map.map[idx] = TileType::Wall;
                }
            }
        }
    }
}