use bracket_lib::{color::{BLACK, BLUE, GREEN, RED, RGB, WHITE, YELLOW}, prelude::{field_of_view, BTerm, Point, VirtualKeyCode}};

use crate::{gui::{self, TargettingMode}, FoV, Map, Position, State};

pub enum TargettingState
{
    None,
    Cancel,
    Selected {path: Vec<Point>, end: Point}
}


pub fn aim_projectile(state : &mut State, ctx : &mut BTerm, start_pos: Point, range : i32) -> TargettingState
{
    match ctx.key
    {
        Some(key) =>
        {
            match key
            {
                VirtualKeyCode::Escape => 
                {
                    return TargettingState::Cancel;
                }
                _ => {}
            }
        },
        None => {}
    }
    let mut available_cells = Vec::new();
    let pos = state.player_pos;
    
    targetting_viewshed(&mut available_cells, state, range, pos, true, ctx);

    let (m_x,m_y) = gui::select_target_mode(state, ctx).to_tuple();

    //let idx = Map::xy_id(m_x, m_y);
    //let mut is_valid_target = false;

    //draws preview of projectile path to targetted tile if the target is in range
    let point = Point::new(m_x, m_y);
    if available_cells.contains(&point)
    {
        let _line = bracket_lib::geometry::Bresenham::new(start_pos, point );
        _line.skip(1).for_each(|pos| 
            {
                ctx.set(pos.x, pos.y, BLACK, GREEN, '*');
            });
        
        ctx.set_bg(point.x, point.y, GREEN);

        if ctx.left_click
        {
            let targets = bracket_lib::geometry::Bresenham::new(start_pos, point).collect();
            return TargettingState::Selected { path:targets, end: point };
        }
        match ctx.key
        {
            Some(key) => 
            {
                if key == VirtualKeyCode::NumpadEnter || key == VirtualKeyCode::Return || key == VirtualKeyCode::F
                {
                    let targets = bracket_lib::geometry::Bresenham::new(start_pos, point).collect();
                    return TargettingState::Selected { path:targets, end: point };
                }
            }
            None => {}
        }
    }
    else
    {
        ctx.set_bg(m_x, m_y,RGB::named(RED));

        if ctx.left_click || ctx.key.unwrap_or(bracket_lib::terminal::VirtualKeyCode::P) == 
            bracket_lib::terminal::VirtualKeyCode::Escape
        {
            return TargettingState::Cancel;
        }
        match ctx.key
        {
            Some(key) => 
            {
                if key == VirtualKeyCode::NumpadEnter || key == VirtualKeyCode::Return || key == VirtualKeyCode::F
                {
                    return TargettingState::Cancel; 
                }
            }
            None => {}
        }
        
    }

    return TargettingState::None;

}



fn targetting_viewshed(valid_tiles : &mut Vec<Point>, state : &mut State, range: i32,
    pos: Point,should_display: bool, ctx : &mut BTerm)
{
    let visible = field_of_view(pos, range, &state.map);

    for idx in visible.iter()
    {
        let distance =
            bracket_lib::pathfinding::DistanceAlg::Pythagoras.distance2d(state.player_pos, *idx);
        if distance <= range as f32
        {
            if should_display
            {
                ctx.set_bg(idx.x, idx.y, RGB::named(BLUE));
            }
            
            valid_tiles.push(*idx);
        }
    }
    
}