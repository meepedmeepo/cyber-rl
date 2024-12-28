use bracket_lib::prelude::{console, DistanceAlg,Point};
use hecs::Entity;
use crate::{ statistics::BaseStatistics, Monster, MAPWIDTH};
use super::{Map,State,FoV,Name,Position,AttackSystem};

pub struct MonsterAI{}
impl MonsterAI
{
    pub fn run(state : &mut State)
    {
        let mut attacking_monsters : Vec<(Entity)> = Vec::new();
        for (_id,(fov,name,position,_monster,stats)) 
        in state.world.query_mut::<(&mut FoV,&Name,&mut Position,&Monster,&BaseStatistics)>()  
        {
            if fov.visible_tiles.contains(&state.player_pos)
            {
                let distance = DistanceAlg::Pythagoras.distance2d(Point::new(position.x,position.y), state.player_pos);
                if distance < 1.5
                {
                    //console::log(&format!("{} shouts curses!",name.name));
                    //AttackSystem::add_attack(_id, , state);
                    //console::log(format!("{} swings at you wildly!",name.name));
                    state.game_log.add_log(format!("{} swings at you wildly!",name.name));
                    attacking_monsters.push(_id);
                }

                let path = bracket_lib::pathfinding::a_star_search
                (Map::xy_id(position.x, position.y) as i32,Map::xy_id(state.player_pos.x, state.player_pos.y) as i32 
                ,  &state.map);
                if path.success && path.steps.len() > 2
                {
                    state.map.blocked[Map::xy_id(position.x,position.y)] = false;
                   
                    position.x = path.steps[1] as i32 % MAPWIDTH;
                    position.y = path.steps[1] as i32 / MAPWIDTH;

                    state.map.blocked[Map::xy_id(position.x,position.y)] = true;
                    fov.dirty = true;
                }
            }
        }
           
        for attacker in attacking_monsters.iter()
        {
            match state.player_ent
            {
                Some(target) =>
                {
                    AttackSystem::add_attack(*attacker, target, state);
                }

                None =>
                {
                    console::log("Player not found!");
                }

            }
        }
    }

}