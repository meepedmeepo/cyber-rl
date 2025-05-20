use bracket_lib::prelude::field_of_view;
use bracket_lib::terminal::*;
use bracket_lib::color;
use macroquad::color::Color;
use crate::components::Equipped;
use crate::gamelog;
use crate::hunger::HungerLevel;
use crate::menus::inventory_state;
use crate::renderer::color_with_alpha;
use crate::renderer::rgb_to_color;
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

use super::keyboard_cursor;
use super::TargettingMode;

pub fn draw_ui(state :&mut State)
{
    
    
}


pub fn draw_gamelog(state :&State)
{
    //ctx.draw_box_double(0, 34, 109, 11, RGB::named(WHITE), RGB::named(BLACK));
    
    //ctx.print_color_centered_at(28,35,color::YELLOW, color::BLACK, "Gamelog:".to_string());

    let mut y = 36;
    for log in state.game_log.view_log(30)
    {
        if !log.is_empty()
        {
            //ctx.print(2, y, log);
            y+=1;
            if y >= 44
            {break;}
        }
    }
}

pub fn draw_status_box(state : &mut State)
{
    //ctx.draw_box_double(78, -1, 31, 46, RGB::named(WHITE), RGB::named(BLACK));
    
    
    let depth = format!("Depth: {}",state.map.depth);
    //ctx.print_color(80, 1, color::YELLOW, color::BLACK, &depth);
    
    //let fps = format!("FPS: {}",ctx.fps);
    //ctx.print_color(90, 1, color::YELLOW, color::BLACK, &fps);

    let turn = format!("Turn: {}",state.turn_number);
    //ctx.print_color(98, 1, color::YELLOW, color::BLACK, &turn);

    let(progress, max) = statistics::get_xp_from_current_level(state);

    for (_id,(_player,stats, bstat, hunger)) in
        state.world.query_mut::<(&Player,&Pools, &BaseStatistics, &HungerLevel)>()
    {
        let health = format!("HP: {} / {} ",stats.hitpoints.current_value,stats.hitpoints.max_value);
        
        //ctx.print_color(88, 4, color::WHITE, color::BLACK, &health);
        //ctx.draw_bar_horizontal(82, 6, 24, stats.hitpoints.current_value, stats.hitpoints.max_value,
         //   color::RED, color::BLACK);

        let xp = format!("Level: {} XP: {} / {} ", stats.level,progress, max);

        //ctx.print_color_centered_at(96, 8, color::WHITE, color::BLACK, &xp);
        //ctx.draw_bar_horizontal(82, 10, 24, progress, max,
        //    color::YELLOW, color::BLACK);
        
        //ctx.print_color_centered_at(94, 12, color::WHITE, color::BLACK, "Hunger");
        //ctx.draw_bar_horizontal(82, 14, 24, hunger.nutrition.current_value, hunger.nutrition.max_value,
        //    color::ORANGE, color::BLACK);
        
        //ctx.print_color(81, 16, color::WHITE, color::BLACK, format!("Armour Class: {}", stats.armour_class.total));

        //ctx.print_color(81, 17, color::WHITE, color::BLACK, format!("Strength: {}", bstat.strength.total));

        //ctx.print_color(81, 18, color::WHITE, color::BLACK, format!("Dexterity: {}", bstat.dexterity.total));
        
        //ctx.print_color(81, 19, color::WHITE, color::BLACK, format!("Intelligence: {}", bstat.intelligence.total));

        //ctx.print_color(81, 20, color::WHITE, color::BLACK, format!("Toughness: {}", bstat.toughness.total));

        //ctx.print_color(81, 21, color::WHITE, color::BLACK, format!("Mental Fortitude {}", bstat.mental_fortitude.total));
    

    }

    //draw_equipped(state, ctx);
    
}


pub fn draw_cursor(pos : Point, state : &mut State, bg :  (u8, u8, u8)) -> Point
{
    let cursor_pos = keyboard_cursor(state,  pos);

    let col = RGB::from_u8(bg.0, bg.1, bg.2);
    state.renderer.draw_square(cursor_pos.x, cursor_pos.y, color_with_alpha(rgb_to_color(col), 0.4));
    
    cursor_pos
}


pub fn draw_equipped(state : &mut State, ctx: &mut BTerm)
{
    let mut items = Vec::new();
    for (ent, (equipped, name)) in state.world.query_mut::<(&Equipped, &Name)>()
    {
        if equipped.owner == state.player_ent.unwrap()
        {
            items.push((equipped.slot, name.name.clone()));
        }
    }

    if items.len() == 0 {return;}
    //sort by equipment slot
    items.sort_by(|a,b| {a.0.cmp(&b.0)});

    let mut btm_pos = 32;
    
    for (_e, name) in items.iter().rev()
    {
        ctx.print(81, btm_pos, name.clone());
        btm_pos -= 1;
    }

    btm_pos -= 1;
    ctx.print(81, btm_pos, "Equipment:");

}