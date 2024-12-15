use bracket_lib::terminal::*;
use bracket_lib::color;
use crate::gamelog;
use crate::menus::inventory_state;
use crate::FoV;
use crate::InContainer;
use crate::Renderable;
use crate::{Player, Statistics,Name,Item};
use super::State;
use std::cmp::{max,min};

pub fn draw_ui(state :&mut State, ctx: &mut BTerm)
{
    ctx.draw_box(0, 42, 76, 7,
         bracket_lib::color::WHITE, bracket_lib::color::BLACK);
    for (_id,(_player,stats)) in
     state.world.query_mut::<(&Player,&Statistics)>()
    {
        let health = format!("HP: {} / {} ",stats.hp,stats.max_hp);
        ctx.print_color(16, 44, color::WHITE, color::BLACK, &health);
        ctx.draw_bar_horizontal(28, 44, 45, stats.hp, stats.max_hp,
             color::RED, color::BLACK);
    }
}

pub fn draw_gamelog(state : &State,ctx: &mut BTerm)
{
    ctx.draw_box(78, 0, 31, 49, RGB::named(WHITE), RGB::named(BLACK));
    let mut y = 3;
    for log in state.game_log.view_log(30)
    {
        if !log.is_empty()
        {
            ctx.print(80, y, log);
            y+=2;
            if y > 48
            {break;}
        }
    }
}


pub fn draw_inventory(state: &mut State, ctx: &mut BTerm)
{
    let mut items = Vec::new();

    for (_id,(_item, _in_container,name)) in state.world.query::<(&Item, &InContainer,&Name)>()
        .iter().filter(|ent| ent.1.1.owner == state.player_ent
        .expect("Couldn't find player entity to query inventory"))
    {
        items.push(name.clone());
    }


    let height = min(38,max(15,items.len()*3));
    ctx.draw_box(22, 10, 35, height, bracket_lib::color::WHITE, bracket_lib::color::BLACK);
    ctx.print_centered_at(35, 11, "Inventory");
    let mut y = 13;
    let mut index : u8 = 97;
    for item in items.iter()
    {
        ctx.print_color(23, y,color::WHITE,color::BLACK, format!("{}.) {}",index as char,item.name.clone()));
        y+=2;
        index += 1;
    }
}


pub fn _ranged_target(state : &mut State, ctx: &mut BTerm, range : i32) 
    -> (inventory_state, Option<Point>)
{
    match ctx.key
    {
        Some(key) =>
        {
            if key == VirtualKeyCode::Escape
            {
                return (inventory_state::Cancel,None);
            }
        }
        None =>{}
    }
    ctx.print_color(5,0,RGB::named(YELLOW), RGB::named(BLACK), "SELECT TARGET:");
    
    let mut available_cells = Vec::new();
    let visible = state.world.get::<&FoV>(state.player_ent
        .expect("Can't find player ent for ranged targetting gui!"));
    match visible
    {
        Ok(ref vis) =>
        {
            for idx in vis.visible_tiles.iter()
            {
                let distance =
                 bracket_lib::pathfinding::DistanceAlg::Pythagoras.distance2d(state.player_pos, *idx);
                 if distance <= range as f32
                 {
                    ctx.set_bg(idx.x, idx.y, RGB::named(BLUE));
                    available_cells.push(idx);
                }
            }
        }

        Err(_) => { return (inventory_state::Cancel,None);}
    }
    //the function version of Bterm.mouse_pos is required to actually get the position!
    let mouse_pos = ctx.mouse_pos();
    let mut is_valid_target = false;

    for idx in available_cells
    {
        if idx.x == mouse_pos.0 && idx.y == mouse_pos.1
        {
            is_valid_target = true;
        }  
    }
    if is_valid_target
    {
        ctx.set_bg(mouse_pos.0,mouse_pos.1,RGB::named(CYAN));
        if ctx.left_click
        {
            return (inventory_state::Selected,Some(Point::new(mouse_pos.0, mouse_pos.1)) );
        }
    }
    else
    {
        ctx.set_bg(mouse_pos.0,mouse_pos.1,RGB::named(RED));
        if ctx.left_click
        {
            return (inventory_state::Cancel, None);
        }
    }

    (inventory_state::None, None)
}