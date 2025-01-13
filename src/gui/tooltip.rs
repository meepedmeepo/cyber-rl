use bracket_lib::{color::{BLACK, LIMEGREEN}, prelude::{BTerm, Point}};

use crate::{Map, Name, Renderable, State, MAPWIDTH};




pub fn draw_tooltip(state : &mut State, ctx : &mut BTerm, cursor_pos : Point )
{

    let idx = Map::xy_id(cursor_pos.x, cursor_pos.y);

    if !state.map.visible_tiles[idx]
    {
        return;
    }

    let content_id = state.map.tile_contents[idx].clone();

    if content_id.len() < 1
    {
        return;
    }

    let mut content = Vec::new();

    for ent in content_id.iter()
    {
        let (name, rend) = 
            state.world.query_one_mut::<(&Name, &Renderable)>(*ent).unwrap();
        
        content.push((name.name.clone(), rend.order));
    }
    content.sort_by_key(|(name, _order)| 0 - name.chars().count() as i32);
    let tip_width = content[0].0.chars().count() as i32 + 5;

    content.sort_by_key(|(_name, order)| -*order);


    let mut tip_pos = cursor_pos;
    //let tip_width = 20;
    let tip_height = content.len() + 3;
    tip_pos.x += 2;

    if tip_pos.x + tip_width >= MAPWIDTH
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