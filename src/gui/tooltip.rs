use std::f32::MIN;

use bracket_lib::{color::{BLACK, LIMEGREEN}, prelude::{BTerm, Point}};

use crate::{camera, Map, Name, Renderable, State,};




pub fn draw_tooltip(state : &mut State, ctx : &mut BTerm, cursor_pos : Point )
{
    let (min_x, _max_x, min_y, _max_y) = camera::get_screen_bounds(state);
    let mut cursor_map_pos = cursor_pos;
    cursor_map_pos.x += min_x;
    cursor_map_pos.y += min_y;

    if cursor_map_pos.x >= state.map.map_width-1 || cursor_map_pos.y >= state.map.map_height -1 || cursor_map_pos.x < 1 || cursor_map_pos.y < 1
    {
        return;
    }
    let map_idx = state.map.xy_idx(cursor_map_pos.x, cursor_map_pos.y);

    let idx = state.map.xy_idx(cursor_pos.x, cursor_pos.y);

    if !state.map.visible_tiles[map_idx]
    {
        return;
    }

    let content_id = state.map.tile_contents[map_idx].clone();

    if content_id.len() < 1
    {
        return;
    }

    let mut content = Vec::new();

    for ent in content_id.iter()
    {
        
        if let Ok((name, rend)) = state.world.query_one_mut::<(&Name, &Renderable)>(*ent)

        {
            content.push((name.name.clone(), rend.order));
        }

        
        
        
    }
    content.sort_by_key(|(name, _order)| 0 - name.chars().count() as i32);
    let tip_width = content[0].0.chars().count() as i32 + 5;

    content.sort_by_key(|(_name, order)| -*order);


    let mut tip_pos = cursor_pos;
    //let tip_width = 20;
    let tip_height = content.len() + 3;
    tip_pos.x += 2;

    if tip_pos.x + tip_width >= state.map.map_width
    {
        //draw to the left of cursor
        tip_pos.x -= tip_width;
        tip_pos.x -= 3;
    } 
    
    ctx.draw_box(tip_pos.x, tip_pos.y-1, tip_width, tip_height, LIMEGREEN, BLACK);

    let mut y = tip_pos.y + 1;
    for (name, _order) in content.iter()
    {
        ctx.print(tip_pos.x + 3, y, name.clone());
        y += 1;
    }


}