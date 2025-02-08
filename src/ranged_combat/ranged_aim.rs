use bracket_lib::{color::{BLACK, BLUE, RED, RGB, WHITE, YELLOW}, prelude::{field_of_view, BTerm, Point, VirtualKeyCode}};
use hecs::Entity;
use macroquad::{color::GREEN, input::{is_key_down, is_key_pressed, is_mouse_button_down, KeyCode, MouseButton}};

use crate::{camera, gui::{self, TargettingMode}, raws::{faction_reaction, Reaction, RAWS}, renderer::color_with_alpha, Faction, FoV, Hidden, Map, Position, State};

pub enum TargettingState
{
    None,
    Cancel,
    Selected {path: Vec<Point>, end: Point}
}


pub fn aim_projectile(state : &mut State, start_pos: Point, range : i32) -> TargettingState
{
    let (min_x, max_x, min_y, max_y) = camera::get_screen_bounds(state);

    if is_key_down(KeyCode::Escape)
    {
        return TargettingState::Cancel
    }
    if is_key_down(KeyCode::Tab)
    {
        if let TargettingMode::Keyboard { cursor_pos } = state.target_mode
                    {
                        state.target_mode = TargettingMode::Keyboard { cursor_pos: 
                            select_nearest_target_pos(state, state.player_ent.unwrap(), cursor_pos) }
                    }
    }
    let mut available_cells = Vec::new();
    let pos = state.player_pos;
    
    targetting_viewshed(&mut available_cells, state, range, pos, true);

    let (m_x,m_y) = gui::select_target_mode(state).to_tuple();

    //let idx = state.map.xy_idx(m_x, m_y);
    //let mut is_valid_target = false;

    //draws preview of projectile path to targetted tile if the target is in range
    let point = Point::new(m_x, m_y);
    let mut screen_point = point;
    screen_point.x -= min_x;
    screen_point.y -= min_y;
    if available_cells.contains(&point)
    {
        let _line = bracket_lib::geometry::Bresenham::new(start_pos, point );
        _line.skip(1).for_each(|pos| 
            {
                let mut screen_pos = pos;
                screen_pos.x -= min_x;
                screen_pos.y -= min_y;

                state.renderer.draw_char_bg(screen_pos.x, screen_pos.y, "*"
                    , macroquad::prelude::BLACK, macroquad::prelude::GREEN);
            });
        

        state.renderer.draw_square(screen_point.x, screen_point.y, color_with_alpha(macroquad::prelude::GREEN, 0.4));

        if is_mouse_button_down(MouseButton::Left)
        {
            let targets = bracket_lib::geometry::Bresenham::new(start_pos, point).collect();
            return TargettingState::Selected { path:targets, end: point };
        }
        if is_key_down(KeyCode::Enter) || is_key_down(KeyCode::KpEnter) || is_key_down(KeyCode::F)
        {
            if point == state.player_pos
            {
                return TargettingState::None;
            }
            let targets = bracket_lib::geometry::Bresenham::new(start_pos, point).collect();
            return TargettingState::Selected { path:targets, end: point };
        }
    }
    else
    {
        state.renderer.draw_square(screen_point.x, screen_point.y, color_with_alpha(macroquad::prelude::RED, 0.4));
        if is_mouse_button_down(MouseButton::Left) || is_key_down(KeyCode::Escape)
        {
            return TargettingState::Cancel;
        }
    }

    return TargettingState::None;

}



fn targetting_viewshed(valid_tiles : &mut Vec<Point>, state : &mut State, range: i32,
    pos: Point,should_display: bool)
{
    let (min_x, max_x, min_y, max_y) = camera::get_screen_bounds(state);
    let visible = field_of_view(pos, range, &state.map);

    for idx in visible.iter()
    {
        let distance =
            bracket_lib::pathfinding::DistanceAlg::Pythagoras.distance2d(state.player_pos, *idx);
        if distance <= range as f32
        {
            if should_display
            {
                let screen_x = idx.x - min_x;
                let screen_y = idx.y - min_y;
                if screen_x > 1 && screen_x < (max_x - min_x)-1 && screen_y > 1 && screen_y < (max_y - min_y) - 1
                {
                    state.renderer.draw_square(idx.x - min_x, idx.y - min_y, color_with_alpha(macroquad::prelude::BLUE, 0.4));
                }
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