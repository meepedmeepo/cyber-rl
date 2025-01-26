use bracket_lib::{color::{BLACK, GRAY, RGB}, prelude::{to_cp437, BTerm, FontCharType}};


use crate::{particles::particle_system, Hidden, Map, Position, Renderable, State, TileType};



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
                    let (glyph, fg, bg) = get_tile_glyph(idx, &state.map);
                    ctx.set(x, y, fg, bg, glyph);
                }
            } else if SHOW_BOUNDARIES
            {
                ctx.set(x, y, RGB::named(GRAY), RGB::named(BLACK), to_cp437('.'));
            }
            x += 1;
        }
        y += 1;
    }

    particle_system::spawn_system(state);
    particle_system::update(state, ctx);

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

            if entity_screen_x > 0 && entity_screen_x < map_width && entity_screen_y > 0 && entity_screen_y < map_height
            {
                ctx.set(entity_screen_x, entity_screen_y, ent.1.fg, ent.1.bg, ent.1.glyph);
            }
        }
    }
}


fn get_tile_glyph(idx : usize, map : &Map) -> (FontCharType, RGB, RGB)
{
    let glyph;
    let mut fg;
    let mut bg = RGB::from_f32(0., 0., 0.);

    match map.map[idx]
    {
        TileType::Floor => 
        {
            glyph = to_cp437('.');
            fg = RGB::from_f32(0., 0.5, 0.5);
        }
        TileType::Wall => 
        {
            let x = idx as i32 % map.map_width;
            let y = idx as i32 / map.map_width;
            glyph = wall_glyph(map, x, y);
            fg = RGB::from_f32(0., 1., 0.);
        }
        TileType::DownStairs =>
        {
            glyph = to_cp437('>');
            fg = RGB::from_f32(0., 1., 1.);
        }
    }

    if !map.visible_tiles[idx]
    {
        fg = fg.to_greyscale();
    }

    (glyph, fg, bg)
}

fn wall_glyph(map : &Map, x: i32, y: i32) -> FontCharType
{
    if x < 1 || x > map.map_width-2 || y < 1 || y > map.map_height-2 as i32 { return 35; }
    let mut mask : u8 = 0;

    if is_revealed_and_wall(map, x, y - 1) { mask +=1; }
    if is_revealed_and_wall(map, x, y + 1) { mask +=2; }
    if is_revealed_and_wall(map, x - 1, y) { mask +=4; }
    if is_revealed_and_wall(map, x + 1, y) { mask +=8; }

    match mask {
        0 => { 9 } // Pillar because we can't see neighbors
        1 => { 186 } // Wall only to the north
        2 => { 186 } // Wall only to the south
        3 => { 186 } // Wall to the north and south
        4 => { 205 } // Wall only to the west
        5 => { 188 } // Wall to the north and west
        6 => { 187 } // Wall to the south and west
        7 => { 185 } // Wall to the north, south and west
        8 => { 205 } // Wall only to the east
        9 => { 200 } // Wall to the north and east
        10 => { 201 } // Wall to the south and east
        11 => { 204 } // Wall to the north, south and east
        12 => { 205 } // Wall to the east and west
        13 => { 202 } // Wall to the east, west, and south
        14 => { 203 } // Wall to the east, west, and north
        15 => { 206 }  // ╬ Wall on all sides
        _ => { 35 } // We missed one?
    }
}

fn is_revealed_and_wall(map : &Map, x: i32, y: i32) -> bool
{
    let idx = map.xy_idx(x, y);

    map.map[idx] == TileType::Wall && map.revealed_tiles[idx]
}

pub fn get_screen_bounds(state : &mut State, ctx : &mut BTerm) -> (i32, i32, i32, i32)
{
    let (x_chars, y_chars) = ctx.get_char_size();

    let center_x = (x_chars / 2) as i32;
    let center_y = (y_chars / 2) as i32;
    let player_pos = state.player_pos;
    
    let min_x = player_pos.x - center_x;
    let max_x = min_x + x_chars as i32;
    let min_y = player_pos.y - center_y;
    let max_y = min_y + y_chars as i32;

    (min_x, max_x, min_y, max_y)
}