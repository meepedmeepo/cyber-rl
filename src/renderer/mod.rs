mod tile_rendering;
mod bmpfonts;
mod resources;


use std::sync::Arc;

use bracket_lib::{color::RGB, prelude::to_cp437};
use macroquad::prelude::*;
pub use resources::*;


#[derive(Debug,PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum RenderBackend {MacroQuad, BracketLib}

#[derive(Clone)]
pub struct Renderer 
{
    pub mode : RenderBackend,
    pub canvas : GraphicGrid,
    pub default_font : Font,
    pub char_size : CharSize,
    pub map_view_size : (u32,u32),
    pub textures : Arc<Resources>
}
#[derive(Debug, Clone, Copy)]
pub struct CharSize(pub i32, pub i32, pub i32);
impl Renderer
{
    pub fn draw_char_bg(&self, x : i32, y: i32, content : u8, fg : Color, bg : Color)
    {
        self.draw_square(x, y, bg);
        self.draw_bmp_char(x, y, fg, content);
    }

    pub fn setup_grid(&mut self)
    {
        self.canvas.tile_width = self.textures.font_width;
        self.canvas.tile_height = self.textures.font_height;
    }

    pub fn draw_char(&self, x : i32, y: i32, content : &str, color : Color )
    {
        // let screen_pos = self.canvas.get_tile_screen_pos(x, y);
        // let draw_pos = self.canvas.get_tile_center_at_coords(screen_pos.0, screen_pos.1,self.canvas.tile_width,self.canvas.tile_height, self.char_size.2);
        // let params = TextParams {color, font_size: (self.canvas.tile_height) as u16
        //     , font: Some(&self.default_font), .. Default::default()};

        // let _size = draw_text_ex(content, draw_pos.0 as f32, draw_pos.1 as f32, params);
        // //println!("width: {} height: {} offset_y: {}",sself.tile_heightize.width, size.height, size.offset_y);

        self.draw_bmp_char(x, y, color, to_cp437(content.chars().next().unwrap()) as u8);


    }

    pub fn draw_square(&self, x : i32, y : i32, color : Color)
    {
        let screen_pos = self.canvas.get_tile_screen_pos(x, y);

        draw_rectangle(screen_pos.0 as f32, screen_pos.1 as f32, self.canvas.tile_width as f32
            , self.canvas.tile_height as f32, color);
    }

    pub fn draw_bmp_char(&self, x : i32, y : i32, color : Color, letter : u8)
    {
        let screen_pos = self.canvas.get_tile_screen_pos(x, y);

        let index = bmpfonts::cp437_to_xy(letter);

        let pos = self.bmpfont_index_to_texture_pos(index);

        let params = DrawTextureParams{source:Some((Rect { x: pos.0 as f32, y: pos.1 as f32
            , w: self.textures.font_width as f32, h: self.textures.font_height as f32 })), ..Default::default()};


        draw_texture_ex(&self.textures.bmp_font, screen_pos.0 as f32, screen_pos.1 as f32, color, params);


    }

    fn bmpfont_index_to_texture_pos(&self, pos : (i32,i32)) -> (i32, i32)
    {
        (pos.0 * self.textures.font_width, pos.1 * self.textures.font_height)
    }
}


pub fn draw_tiles(rend : &Renderer)
{
    let w = rend.canvas.tile_width as f32;
    let h = rend.canvas.tile_height as f32;
    
    for y in 1..rend.canvas.grid_height
    {
        for x in 1..rend.canvas.grid_width
        {
            rend.draw_square( x , y, GREEN);
            draw_rectangle_lines(x as f32*w, y as f32*h, w, h, 1., BLACK);
            rend.draw_char(x, y, ".", BLACK);

        }
    }
    //rend.draw_char(1, 0, "x", BLACK);
}

#[derive(Debug, Clone, Copy)]
pub struct GraphicGrid
{
    pub tile_width : i32,
    pub tile_height : i32,
    pub grid_width : i32,
    pub grid_height : i32,
}

impl GraphicGrid
{
    pub fn new(tile_width : i32, tile_height : i32, grid_width : i32, grid_height : i32) -> GraphicGrid
    {
        GraphicGrid { tile_width, tile_height, grid_width, grid_height }
    }
    
    pub fn get_tile_center_at_coords(&self, x : i32, y : i32, w : i32, h : i32, o : i32) -> (i32, i32)
{
    (x+(self.tile_width - (self.tile_width/2)), y+(self.tile_height - (self.tile_height/2) ))
}

//Gets screen location of a grid tile
pub fn get_tile_screen_pos(&self, x : i32, y : i32) -> (i32, i32)
{
    (x * self.tile_width, y * self.tile_height)
}

///Gets xy grid position of a screenlocation
pub fn get_tile_coords(&self, x : i32, y : i32) -> (i32, i32)
{
    (x % self.tile_width, y % self.tile_height)
}

}

pub fn rgb_to_color(value : RGB) -> Color
{
    Color { r: value.r, g: value.g, b: value.b, a: 1.0 }
}

pub fn color_with_alpha(color : Color, alpha : f32) -> Color
{
    let mut col = color;
    col.a = alpha;

    col
}