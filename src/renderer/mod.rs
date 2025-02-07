mod tile_rendering;
use macroquad::prelude::*;


#[derive(Debug,PartialEq, Eq, PartialOrd, Ord)]
enum RenderBackend {MacroQuad, BracketLib}


struct Renderer 
{
    pub mode : RenderBackend
}


pub fn draw_tiles()
{
    let w = 40.;
    let h = 40.;

    for y in 0..11
    {
        for x in 0..11
        {
            draw_rectangle_lines(x as f32*w, y as f32*h, w, h, 1., GREEN);
        }
    }
}


pub struct GraphicGrid
{
    pub tile_width : i32,
    pub tile_height : i32,
    pub grid_width : i32,
    pub grid_height : i32,
}

impl GraphicGrid
{
    pub fn get_tile_center_at_coords(&self, x : i32, y : i32) -> (i32, i32)
{
    (x+(self.tile_width/2), y + (self.tile_height/2))
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

