

use bracket_lib::{ prelude::{field_of_view, Algorithm2D, Point}};
use macroquad::{color::YELLOW, input::{is_key_down, is_mouse_button_down, KeyCode, MouseButton}, miniquad::gl::GL_BLUE, text::draw_text_ex};

use crate::{camera, menus::inventory_state, renderer::color_with_alpha, FoV, State};


pub enum TargettingMode
{
    Mouse,
    Keyboard{cursor_pos : Point}
}

pub fn keyboard_cursor(state : &mut State, pos : Point) -> Point
{
    let translation = key_to_translation();

    if state.map.in_bounds(pos+translation)
    {
        return pos+translation
    }
    else
    {
        pos
    }
}


pub fn key_to_translation() -> Point
{
    if is_key_down(KeyCode::Kp8) {return Point::new(0, -1)}
    if is_key_down(KeyCode::Kp9) {return Point::new(1, -1)}
    if is_key_down(KeyCode::Kp6) {return Point::new(1, 0)}
    if is_key_down(KeyCode::Kp3) {return Point::new(1, 1)}
    if is_key_down(KeyCode::Kp2) {return Point::new(0, 1)}
    if is_key_down(KeyCode::Kp1) {return Point::new(-1, 1)}
    if is_key_down(KeyCode::Kp4) {return Point::new(-1, 0)}
    if is_key_down(KeyCode::Kp7) {return Point::new(-1, -1)}

    Point::zero()
}


pub fn mouse_cursor(state : &mut State) -> Point
{
    let p = macroquad::input::mouse_position();

    Point::from_tuple(state.renderer.canvas.get_tile_coords(p.0 as i32, p.1 as i32))
}


pub fn select_target_mode(state : &mut State) -> Point
{
    match state.target_mode
    {
        TargettingMode::Keyboard{cursor_pos: pos}=>
        { 
            let point = keyboard_cursor(state, pos); 
            state.target_mode = TargettingMode::Keyboard { cursor_pos: point };
            return point;
        }
        TargettingMode::Mouse=>{ return mouse_cursor(state);}
    }
}

pub fn ranged_target(state : &mut State, range : i32, aoe : Option<i32>) 
    -> (inventory_state, Option<Point>)
{
    let (min_x, max_x, min_y, max_y) = camera::get_screen_bounds(state);

    if is_key_down(KeyCode::Escape)
    {
        return (inventory_state::Cancel, None);
    }

    state.renderer.draw_char(5, 0, "SELECT TARGET", macroquad::prelude::YELLOW);
    
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
                    let screen_x = idx.x - min_x;
                    let screen_y = idx.y - min_y;
                    if screen_x > 1 && screen_x < (max_x - min_x)-1 && screen_y > 1 && screen_y < (max_y - min_y) - 1
                    {
                        let mut bg = macroquad::prelude::BLUE;
                        bg.a = 0.4;
                        state.renderer.draw_square(screen_x, screen_y, bg);
                        available_cells.push(idx.clone());
                    }
                }
            }
        }

        Err(_) => { return (inventory_state::Cancel,None);}
    }
    std::mem::drop(visible);
    //the function version of Bterm.mouse_pos is required to actually get the position!
    
    let mouse_pos = select_target_mode(state).to_tuple();

    let mut mouse_map_pos = mouse_pos;
    mouse_map_pos.0 += min_x;
    mouse_map_pos.1 += min_y;

    let mut is_valid_target = false;

    for idx in available_cells
    {
        if idx.x == mouse_map_pos.0 && idx.y == mouse_map_pos.1
        {
            is_valid_target = true;
        }  
    }
    if is_valid_target
    {
        state.renderer.draw_square(mouse_pos.0, mouse_pos.1, color_with_alpha(macroquad::color::SKYBLUE, 0.4));
        if is_mouse_button_down(MouseButton::Left)
        {
            return (inventory_state::Selected,Some(Point::new(mouse_map_pos.0, mouse_map_pos.1)) );
        }

        if is_key_down(KeyCode::Enter) || is_key_down(KeyCode::KpEnter) || is_key_down(KeyCode::F)
        {
            return (inventory_state::Selected,Some(Point::new(mouse_map_pos.0, mouse_map_pos.1)));
        }
    }
    else
    {
        let mut bg = macroquad::prelude::RED;
        bg.a = 0.4;
        state.renderer.draw_square(mouse_pos.0, mouse_pos.1, bg);
        if is_mouse_button_down(MouseButton::Left)
        {
            return (inventory_state::Cancel, None);
        }

        if is_key_down(KeyCode::Enter) || is_key_down(KeyCode::KpEnter) || is_key_down(KeyCode::F)
        {
            return (inventory_state::Cancel, None);
        }
    }

    match aoe
    {
        Some(radius) => 
        {
            let tiles = field_of_view(Point::from_tuple(mouse_map_pos),
                radius, &state.map);
            
            for point in tiles.iter()
            {
                let screen_x = point.x - min_x;
                let screen_y = point.y - min_y;

                let mut bg = YELLOW;
                bg.a = 0.4;
                state.renderer.draw_square(screen_x, screen_y, bg);
            }
            
        }

        None => {}
    }

    (inventory_state::None, None)
}