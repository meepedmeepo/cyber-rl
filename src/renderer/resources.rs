use std::sync::Arc;

use macroquad::texture::Texture2D;



#[derive(Clone)]
pub struct Resources 
{
    pub bmp_font : Arc<Texture2D>,
    pub font_width : i32,
    pub font_height : i32,
}