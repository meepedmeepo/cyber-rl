use bracket_lib::prelude::field_of_view;
use bracket_lib::terminal::*;
use bracket_lib::color;
use crate::gamelog;
use crate::menus::inventory_state;
use crate::statistics;
use crate::statistics::Pools;
use crate::AoE;
use crate::FoV;
use crate::InContainer;
use crate::Renderable;
use crate::{Player, Name,Item};
use crate::State;
use std::cmp::{max,min};

pub fn draw_ui(state :&mut State, ctx: &mut BTerm)
{
   // ctx.draw_box_double(0, 42, 76, 7,bracket_lib::color::WHITE, bracket_lib::color::BLACK);
    
    
}


pub fn draw_gamelog(state :&State, ctx: &mut BTerm)
{
    ctx.draw_box_double(0, 49, 109, 30, RGB::named(WHITE), RGB::named(BLACK));
    
    ctx.print_color_centered_at(28,51,color::YELLOW, color::BLACK, "Gamelog:".to_string());

    let mut y = 53;
    for log in state.game_log.view_log(30)
    {
        if !log.is_empty()
        {
            ctx.print(2, y, log);
            y+=2;
            if y > 78
            {break;}
        }
    }
}

pub fn draw_status_box(state : &mut State,ctx: &mut BTerm)
{
    ctx.draw_box_double(78, -1, 31, 50, RGB::named(WHITE), RGB::named(BLACK));
    
    
    let depth = format!("Depth: {}",state.map.depth);
    ctx.print_color(82, 1, color::YELLOW, color::BLACK, &depth);
    
    let fps = format!("FPS: {}",ctx.fps);
    ctx.print_color(100, 1, color::YELLOW, color::BLACK, &fps);

    let(progress, max) = statistics::get_xp_from_current_level(state);

    for (_id,(_player,stats)) in
     state.world.query_mut::<(&Player,&Pools)>()
    {
        let health = format!("HP: {} / {} ",stats.hitpoints.current_value,stats.hitpoints.max_value);
        
        ctx.print_color(88, 4, color::WHITE, color::BLACK, &health);
        ctx.draw_bar_horizontal(82, 6, 24, stats.hitpoints.current_value, stats.hitpoints.max_value,
             color::RED, color::BLACK);

        
        

        let xp = format!("Level: {} XP: {} / {} ", stats.level,progress, max);

        ctx.print_color_centered_at(96, 8, color::WHITE, color::BLACK, &xp);
        ctx.draw_bar_horizontal(82, 10, 24, progress, max,
             color::YELLOW, color::BLACK);
        
    }
    
}



