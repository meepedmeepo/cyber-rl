

use bracket_lib::{color::{BLACK, BLUE, CYAN, RED, RGB, YELLOW}, prelude::{field_of_view, Algorithm2D, BTerm, Point, VirtualKeyCode}};

use crate::{menus::inventory_state, FoV, State};


pub enum TargettingMode
{
    Mouse,
    Keyboard{cursor_pos : Point}
}

pub fn keyboard_cursor(state : &mut State, ctx: &mut BTerm, pos : Point) -> Point
{
    let translation = key_to_translation(ctx);

    if state.map.in_bounds(pos+translation)
    {
        return pos+translation
    }
    else
    {
        pos
    }
}
   

pub fn key_to_translation(ctx: &mut BTerm) -> Point
{
    match ctx.key
    {
        Some(key) => 
        {
            match key
            {
                VirtualKeyCode::Numpad8 => {Point::new(0, -1)}
                VirtualKeyCode::Numpad9 => {Point::new(1, -1)}
                VirtualKeyCode::Numpad6 => {Point::new(1, 0)}
                VirtualKeyCode::Numpad3 => {Point::new(1, 1)}
                VirtualKeyCode::Numpad2 => {Point::new(0, 1)}
                VirtualKeyCode::Numpad1 => {Point::new(-1, 1)}
                VirtualKeyCode::Numpad4 => {Point::new(-1, 0)}
                VirtualKeyCode::Numpad7 => {Point::new(-1, -1)}
                _ => {Point::new(0, 0)}
            }
        }
        None => {Point::new(0, 0)}
    }
}


pub fn mouse_cursor(state : &mut State, ctx: &mut BTerm) -> Point
{
    ctx.mouse_point()
}


pub fn select_target_mode(state : &mut State, ctx: &mut BTerm) -> Point
{
    match state.target_mode
    {
        TargettingMode::Keyboard{cursor_pos: pos}=>
        { 
            let point = keyboard_cursor(state, ctx, pos); 
            state.target_mode = TargettingMode::Keyboard { cursor_pos: point };
            return point;
        }
        TargettingMode::Mouse=>{ return mouse_cursor(state, ctx);}
    }
}

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
                    available_cells.push(idx.clone());
                }
            }
        }

        Err(_) => { return (inventory_state::Cancel,None);}
    }
    std::mem::drop(visible);
    //the function version of Bterm.mouse_pos is required to actually get the position!
    
    let mouse_pos = select_target_mode(state, ctx).to_tuple();
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

        match ctx.key
        {
            Some(key) => 
            {
                if key == VirtualKeyCode::NumpadEnter || key == VirtualKeyCode::End || key == VirtualKeyCode::F
                {
                    return (inventory_state::Selected,Some(Point::new(mouse_pos.0, mouse_pos.1)));
                }
            }
            None => {}
        }
    }
    else
    {
        ctx.set_bg(mouse_pos.0,mouse_pos.1,RGB::named(RED));
        if ctx.left_click
        {
            return (inventory_state::Cancel, None);
        }

        match ctx.key
        {
            Some(key) => 
            {
                if key == VirtualKeyCode::NumpadEnter || key == VirtualKeyCode::End ||key == VirtualKeyCode::F
                {
                    return (inventory_state::Cancel, None);
                }
            }
            None => {}
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