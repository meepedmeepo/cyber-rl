use std::borrow::Cow;

use bracket_lib::{color::{BLACK, GRAY, RGB}, prelude::{to_char, to_cp437, BTerm, FontCharType}};
use codepage_437::CP437_CONTROL;
use encode_unicode::{StrExt, Utf16Char};
use codepage_437::*;

use crate::{particles::particle_system, renderer::rgb_to_color, Hidden, Map, Position, Renderable, State, TileType};

mod themes;

use themes::*;


const SHOW_BOUNDARIES : bool = false;

pub fn render_camera(state : &mut State)
{
    let (x_chars, y_chars) = state.renderer.map_view_size;

    let center_x = (x_chars / 2) as i32;
    let center_y = (y_chars / 2) as i32;
    let player_pos = state.player_pos;
    
    let min_x = player_pos.x - center_x;
    let max_x = min_x + x_chars as i32;
    let min_y = player_pos.y - center_y;
    let max_y = min_y + y_chars as i32;


    let map_width = state.map.map_width - 1;
    let map_height = state.map.map_height - 1;

    let mut y = 0;
    for ty in min_y .. max_y
    {
        let mut x = 0;
        for tx in min_x .. max_x
        {
            if tx > 0 && tx < map_width && ty > 0 && ty < map_height
            {
                let idx = state.map.xy_idx(tx, ty);
                if state.map.revealed_tiles[idx]
                {
                    let (glyph, fg, bg) = tile_glyph(idx, &state.map);
                    //let dt = glyph.to_string();
                    //let data =  dt.into_cp437(&CP437_CONTROL).unwrap();
                    //let res = 
                    //let mut buf = [0;3];
                    let cont = *glyph.to_cp437(&CP437_WINGDINGS).unwrap().first().unwrap();
                    state.renderer.draw_char_bg(x, y,cont, rgb_to_color(fg), rgb_to_color(bg));
                }
            } else if SHOW_BOUNDARIES
            {
                state.renderer.draw_char_bg(x, y, *".".to_cp437(&CP437_WINGDINGS).unwrap().first().unwrap(), rgb_to_color(RGB::named(GRAY))
                    , rgb_to_color(RGB::named(BLACK)));
            }
            x += 1;
        }
        y += 1;
    }

    particle_system::spawn_system(state);
    particle_system::update(state);

    let mut entities_to_render  = 
        state.world.query_mut::<(&Position,&Renderable)>().without::<&Hidden>()
        .into_iter()
        .map(|ent|{(ent.1.0,ent.1.1)})
        .collect::<Vec<_>>();

    
    entities_to_render.sort_by_key(|a| a.1.order);

    for ent in entities_to_render
    {
        let idx = state.map.xy_idx(ent.0.x, ent.0.y);
        if state.map.visible_tiles[idx]
    { 
            let entity_screen_x = ent.0.x - min_x;
            let entity_screen_y = ent.0.y - min_y;

            if entity_screen_x > 0 && entity_screen_x < max_x -2 && entity_screen_y > 0 && entity_screen_y < max_y - 2
            {
                let fg = rgb_to_color(ent.1.fg);
                let bg = rgb_to_color(ent.1.bg);
                let cont = *ent.1.glyph.to_cp437(&CP437_WINGDINGS).unwrap().first().unwrap();
                state.renderer.draw_char_bg(entity_screen_x, entity_screen_y, cont,fg , bg);
            }
        }
    }
}





pub fn get_screen_bounds(state : &mut State) -> (i32, i32, i32, i32)
{
    let (x_chars, y_chars) = state.renderer.map_view_size;

    let center_x = (x_chars / 2) as i32;
    let center_y = (y_chars / 2) as i32;
    let player_pos = state.player_pos;
    
    let min_x = player_pos.x - center_x;
    let max_x = min_x + x_chars as i32;
    let min_y = player_pos.y - center_y;
    let max_y = min_y + y_chars as i32;

    (min_x, max_x, min_y, max_y)
}