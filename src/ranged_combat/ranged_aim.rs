use bracket_lib::{color::{BLACK, BLUE, GREEN, RED, RGB, WHITE, YELLOW}, prelude::{field_of_view, BTerm, Point}};

use crate::{FoV, Map, Position, State};

pub enum TargettingState
{
    None,
    Cancel,
    Selected {target: Point}
}


pub fn aim_projectile(state : &mut State, ctx : &mut BTerm, start_pos: Point, range : i32) -> TargettingState
{
    let mut available_cells = Vec::new();
    let pos = state.player_pos;
    
    targetting_viewshed(&mut available_cells, state, range, pos, true, ctx);

    let (m_x,m_y) = ctx.mouse_pos();

    //let idx = Map::xy_id(m_x, m_y);
    //let mut is_valid_target = false;

    //draws preview of projectile path to targetted tile if the target is in range
    let point = Point::new(m_x, m_y);
    if available_cells.contains(&point)
    {
        let _line = bracket_lib::geometry::Bresenham::new(start_pos, point )
            .for_each(|pos| 
            {
                ctx.set(pos.x, pos.y, BLACK, GREEN, '*');
            });
        
        ctx.set_bg(point.x, point.y, GREEN);

        if ctx.left_click
        {
            return TargettingState::Selected { target: point };
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