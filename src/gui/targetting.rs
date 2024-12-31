

use bracket_lib::{color::{BLACK, BLUE, CYAN, RED, RGB, YELLOW}, prelude::{field_of_view, BTerm, Point, VirtualKeyCode}};

use crate::{menus::inventory_state, FoV, State};

pub fn ranged_target(state : &mut State, ctx: &mut BTerm, range : i32, aoe : Option<i32>) 
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

    match aoe
    {
        Some(radius) => 
        {
            let tiles = field_of_view(Point::from_tuple(mouse_pos),
                radius, &state.map);
            
            for point in tiles.iter()
            {
                ctx.set_bg(point.x, point.y, YELLOW);
            }
            
        }

        None => {}
    }

    (inventory_state::None, None)
}