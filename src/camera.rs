use bracket_lib::prelude::BTerm;

use crate::State;



const SHOW_BOUNDARIES : bool = true;

pub fn render_camera(state : &mut State, ctx : &mut BTerm)
{
    let (x_chars, y_chars) = ctx.get_char_size();

    let center_x = (x_chars / 2) as i32;
    let center_y = (y_chars / 2) as i32;
    let player_pos = state.player_pos;
    
    let min_x = player_pos.x - center_x;
    let max_x = min_x + x_chars as i32;
    let min_y = player_pos.y - center_y;
    let max_y = min_y + y_chars as i32;


    let map_width = map.width - 1;
    let map_height = map.height - 1;
}
