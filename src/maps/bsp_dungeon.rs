use bracket_lib::{prelude::Rect, random::RandomNumberGenerator};

use super::{common::apply_room_to_map, BuilderMap, InitialMapBuilder, Map, TileType};




pub struct BspDungeon 
{
    rects : Vec<Rect>,
}

impl InitialMapBuilder for BspDungeon
{
    fn build_map(&mut self, rng: &mut bracket_lib::prelude::RandomNumberGenerator, build_data: &mut super::BuilderMap) 
    {
        self.build(rng, build_data);
    }
}

impl BspDungeon
{
    pub fn new() -> Box<BspDungeon>
    {
        Box::new(BspDungeon { rects: Vec::new() })
    }

    fn build(&mut self, rng : &mut RandomNumberGenerator, build_data : &mut BuilderMap)
    {
        let mut rooms: Vec<Rect> = Vec::new();
        self.rects.clear();
        self.rects.push(Rect::with_size(2, 2, build_data.map.map_width-5, build_data.map.map_height-5));

        let first_room = self.rects[0];
        self.add_subrects(first_room);//split the first room into 4 rects

        //up to 240 times we take a random rectangle and split it. If a room can be built in there, then the room is added to the list

        let mut n_rooms = 0;
        while n_rooms < 240
        {
            let rect = self.random_rect(rng);
            let candidate = self.get_random_sub_rect(rect, rng);

            if self.is_possible(candidate, &mut build_data.map)
            {
                apply_room_to_map(&mut build_data.map, &candidate);
                rooms.push(candidate);
                self.add_subrects(rect);

            }

            n_rooms += 1;
        }

        build_data.rooms = Some(rooms);

    }

    fn add_subrects(&mut self, rect : Rect)
    {
        let width = i32::abs(rect.x1 - rect.x2);
        let height = i32::abs(rect.y1 - rect.y2);
        let half_width =  i32::max(width/2, 1);
        let half_height = i32::max(height / 2, 1);

        self.rects.push(Rect::with_size( rect.x1, rect.y1, half_width, half_height ));
        self.rects.push(Rect::with_size( rect.x1, rect.y1 + half_height, half_width, half_height ));
        self.rects.push(Rect::with_size( rect.x1 + half_width, rect.y1, half_width, half_height ));
        self.rects.push(Rect::with_size( rect.x1 + half_width, rect.y1 + half_height, half_width, half_height ));
    }

    fn random_rect(&mut self, rng : &mut RandomNumberGenerator) -> Rect
    {
        if self.rects.len() == 1 {return self.rects[0];}
        let idx = (rng.roll_dice(1, (self.rects.len() as i32) - 1)) as usize;

        self.rects[idx]
    }

    fn get_random_sub_rect(&self, rect : Rect, rng : &mut RandomNumberGenerator) -> Rect {
        let mut result = rect;
        let rect_width = i32::abs(rect.x1 - rect.x2);
        let rect_height = i32::abs(rect.y1 - rect.y2);

        let w = i32::max(3, rng.roll_dice(1, i32::min(rect_width, 10))-1) + 1;
        let h = i32::max(3, rng.roll_dice(1, i32::min(rect_height, 10))-1) + 1;

        result.x1 += rng.roll_dice(1, 6)-1;
        result.y1 += rng.roll_dice(1, 6)-1;
        result.x2 = result.x1 + w;
        result.y2 = result.y1 + h;

        result
    }

    fn is_possible(&self, rect : Rect, map : &Map) -> bool
    {
        let mut expanded = rect;
        expanded.x1-=2;
        expanded.x2 +=2;
        expanded.y1-=2;
        expanded.y2+=2;

        let mut can_build = true;

        for y in expanded.y1 .. expanded.y2
        {
            for x in expanded.x1 .. expanded.x2
            {
                if x > map.map_width-2 {can_build = false;}
                if y > map.map_height-2 {can_build = false;}
                if x < 1 {can_build = false;}
                if y < 1 {can_build = false;}
                if can_build
                {
                    let idx = map.xy_idx(x, y);
                    if map.map[idx] != TileType::Wall
                    {
                        can_build = false
                    }
                }
            }
        }

        can_build
    }
}
