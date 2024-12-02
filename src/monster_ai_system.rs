use bracket_lib::prelude::{console, DistanceAlg,Point};
use crate::Monster;
use super::{Map,State,FoV,Name,Position};

pub struct MonsterAI{}
impl MonsterAI
{
    pub fn run(state : &mut State)
    {
        for (_id,(fov,name,position,_monster)) 
        in state.world.query_mut::<(&mut FoV,&Name,&mut Position,&Monster)>()  
        {
            if fov.visible_tiles.contains(&state.player_pos)
            {
                let distance = DistanceAlg::Pythagoras.distance2d(Point::new(position.x,position.y), state.player_pos);
                if distance < 1.5
                {
                    console::log(&format!("{} shouts curses!",name.name));
                }

                let path = bracket_lib::pathfinding::a_star_search
                (Map::xy_id(position.x, position.y) as i32,Map::xy_id(state.player_pos.x, state.player_pos.y) as i32 
                ,  &state.map);
                if path.success && path.steps.len() > 2
                {
                    state.map.blocked[Map::xy_id(position.x,position.y)] = false;
                   
                    position.x = path.steps[1] as i32 % state.map.width;
                    position.y = path.steps[1] as i32 / state.map.width;

                    state.map.blocked[Map::xy_id(position.x,position.y)] = true;
                    fov.dirty = true;
                }
            }

        }   
    }

}