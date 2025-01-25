use bracket_lib::random::RandomNumberGenerator;

use super::{BuilderMap, MetaMapBuilder};


pub enum SortBy {LeftMost}

pub struct RoomSorter { sorting : SortBy}

impl MetaMapBuilder for RoomSorter
{
    fn build_map(&mut self, rng: &mut bracket_lib::prelude::RandomNumberGenerator, build_data: &mut super::BuilderMap) 
    {
        self.sorter(rng, build_data);
    }
}

impl RoomSorter
{
    pub fn new() -> Box<RoomSorter>
    {
        Box::new(RoomSorter {sorting: SortBy::LeftMost})
    }

    fn sorter(&mut self, _rng : &mut RandomNumberGenerator, build_data : &mut BuilderMap)
    {
        match self.sorting
        {
            SortBy::LeftMost => build_data.rooms.as_mut().unwrap().sort_by(|a,b| a.x1.cmp(&b.x1)),
        }
    }
}