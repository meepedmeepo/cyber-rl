use bracket_lib::{prelude::{DistanceAlg, Point}, random::RandomNumberGenerator};

use super::{tile_walkable, BuilderMap, MetaMapBuilder, TileType};

#[allow(dead_code)]
pub enum XStart {LEFT, CENTER, RIGHT}

#[allow(dead_code)]
pub enum YStart {TOP, CENTER, BOTTOM}

pub struct AreaStartingPosition 
{
    x : XStart,
    y : YStart
}

impl MetaMapBuilder for AreaStartingPosition
{
    fn build_map(&mut self, rng: &mut bracket_lib::prelude::RandomNumberGenerator, build_data: &mut super::BuilderMap) 
    {
        self.build(rng, build_data);
    }
}

impl AreaStartingPosition
{
    #[allow(dead_code)]
    pub fn new(x : XStart, y: YStart) -> Box<AreaStartingPosition>
    {
        Box::new(AreaStartingPosition {x, y})
    }

    fn build(&mut self, _rng : &mut RandomNumberGenerator, build_data : &mut BuilderMap)
    {
        let seed_x;
        let seed_y;

        match self.x {
            XStart::LEFT => seed_x = 1,
            XStart::CENTER => seed_x = build_data.map.map_width / 2,
            XStart::RIGHT => seed_x = build_data.map.map_width - 2
        }

        match self.y 
        {
            YStart::TOP => seed_y = 1,
            YStart::CENTER => seed_y = build_data.map.map_height / 2,
            YStart::BOTTOM => seed_y = build_data.map.map_height - 2
        }

        let mut available_floors : Vec<(usize, f32)> = Vec::new();
        for (idx, tiletype) in build_data.map.map.iter().enumerate()
        {
            if tile_walkable(*tiletype)
            {
                available_floors.push((idx, 
                    DistanceAlg::PythagorasSquared
                    .distance2d(Point::new(idx as i32 % build_data.map.map_width, idx as i32 / build_data.map.map_width)
                    , Point::new(seed_x, seed_y) )));
            }
        }

        if available_floors.is_empty()
        {
            panic!("No valid floors to start on!");
        }

        available_floors.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let start_x = available_floors[0].0 as i32 % build_data.map.map_width;
        let start_y = available_floors[0].0 as i32 / build_data.map.map_width;

        build_data.starting_position = Some(Point{x : start_x, y: start_y});
    }
}