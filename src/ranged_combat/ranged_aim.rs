use bracket_lib::{color::{BLACK, BLUE, GREEN, RED, RGB, WHITE, YELLOW}, prelude::{field_of_view, BTerm, Point, VirtualKeyCode}};
use hecs::Entity;

use crate::{gui::{self, TargettingMode}, raws::{faction_reaction, Reaction, RAWS}, Faction, FoV, Hidden, Map, Position, State};

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
                VirtualKeyCode::Tab =>
                {
                    if let TargettingMode::Keyboard { cursor_pos } = state.target_mode
                    {
                        state.target_mode = TargettingMode::Keyboard { cursor_pos: 
                            select_nearest_target_pos(state, state.player_ent.unwrap(), cursor_pos) }
                    }
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
                    if point == state.player_pos
                    {
                        return TargettingState::None;
                    }
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

pub fn select_nearest_target_pos(state : &mut State, ent : Entity, current_pos : Point ) -> Point
{
    let pos  = *state.world.get::<&Position>(ent).unwrap().clone();
    let mut point = pos.into();
    
    let faction = state.world.get::<&Faction>(ent).unwrap().name.clone();

    let fov = state.world.get::<&FoV>(ent).unwrap().visible_tiles.clone();

    let mut closest = Point::zero();

    for i in fov.iter()
    {
        let cont = state.map.get_mob_entities_at_position(state, *i);
        for mob in cont.iter()
        {
            if let Ok(_) = state.world.get::<&Hidden>(*mob)
            {
                continue;
            }
            let their_fac = state.world.get::<&Faction>(*mob).unwrap().name.clone();
            let their_pos  = *state.world.get::<&Position>(*mob).unwrap().clone();
            let their_point = their_pos.into();
            let react = faction_reaction(&their_fac, &faction, &RAWS.lock().unwrap());

            if react == Reaction::Attack && their_point != current_pos && (closest == Point::zero() || 
                bracket_lib::pathfinding::DistanceAlg::Pythagoras.distance2d(point, their_point) < 
                bracket_lib::pathfinding::DistanceAlg::Pythagoras.distance2d(point, closest) )
            {
                closest = their_point;
            }
        }
        
    }

    if closest != Point::zero()
    {
        point = closest;
    }



    point
}