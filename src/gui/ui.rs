use bracket_lib::prelude::field_of_view;
use bracket_lib::terminal::*;
use bracket_lib::color;
use crate::gamelog;
use crate::hunger::HungerLevel;
use crate::menus::inventory_state;
use crate::statistics;
use crate::statistics::BaseStatistics;
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
    ctx.draw_box_double(0, 34, 109, 11, RGB::named(WHITE), RGB::named(BLACK));
    
    ctx.print_color_centered_at(28,35,color::YELLOW, color::BLACK, "Gamelog:".to_string());

    let mut y = 36;
    for log in state.game_log.view_log(30)
    {
        if !log.is_empty()
        {
            ctx.print(2, y, log);
            y+=1;
            if y >= 44
            {break;}
        }
    }
}

pub fn draw_status_box(state : &mut State,ctx: &mut BTerm)
{
    ctx.draw_box_double(78, -1, 31, 46, RGB::named(WHITE), RGB::named(BLACK));
    
    
    let depth = format!("Depth: {}",state.map.depth);
    ctx.print_color(82, 1, color::YELLOW, color::BLACK, &depth);
    
    let fps = format!("FPS: {}",ctx.fps);
    ctx.print_color(100, 1, color::YELLOW, color::BLACK, &fps);

    let(progress, max) = statistics::get_xp_from_current_level(state);

    for (_id,(_player,stats, bstat, hunger)) in
     state.world.query_mut::<(&Player,&Pools, &BaseStatistics, &HungerLevel)>()
    {
        let health = format!("HP: {} / {} ",stats.hitpoints.current_value,stats.hitpoints.max_value);
        
        ctx.print_color(88, 4, color::WHITE, color::BLACK, &health);
        ctx.draw_bar_horizontal(82, 6, 24, stats.hitpoints.current_value, stats.hitpoints.max_value,
             color::RED, color::BLACK);

        
        

        let xp = format!("Level: {} XP: {} / {} ", stats.level,progress, max);

        ctx.print_color_centered_at(96, 8, color::WHITE, color::BLACK, &xp);
        ctx.draw_bar_horizontal(82, 10, 24, progress, max,
             color::YELLOW, color::BLACK);
        
        ctx.print_color_centered_at(94, 12, color::WHITE, color::BLACK, "Hunger");
        ctx.draw_bar_horizontal(82, 14, 24, hunger.nutrition.current_value, hunger.nutrition.max_value,
            color::ORANGE, color::BLACK);
        
        ctx.print_color(81, 17, color::WHITE, color::BLACK, format!("Strength: {}", bstat.strength.total));

        ctx.print_color(81, 18, color::WHITE, color::BLACK, format!("Dexterity: {}", bstat.dexterity.total));
        
        ctx.print_color(81, 19, color::WHITE, color::BLACK, format!("Intelligence: {}", bstat.intelligence.total));

        ctx.print_color(81, 20, color::WHITE, color::BLACK, format!("Toughness: {}", bstat.toughness.total));

        ctx.print_color(81, 21, color::WHITE, color::BLACK, format!("Mental Fortitude {}", bstat.mental_fortitude.total));
    }
    
}



