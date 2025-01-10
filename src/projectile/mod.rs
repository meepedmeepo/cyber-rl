use bracket_lib::{color::RGB, prelude::Point};
use queues::Queue;

pub mod projectile_system;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ProjectileType
{
    Beam,
    Missile

}
pub struct Projectile
{
    pub current_frame_time : f32,
    pub frame_time : f32,
    pub path : Queue<Point>,
    //temp
    pub dmg : i32
}

pub struct ProjectileRequest
{
    pub frame_time : f32,
    pub path : Vec<Point>,
    pub projectile_type : ProjectileType,
    pub dmg : i32,

    pub glyph : char,
    pub fg : RGB,
    pub bg : RGB,
    pub order : i32,
}

impl ProjectileRequest
{
    fn new(frame_time: f32, path: Vec<Point>, projectile_type: ProjectileType, glyph: char, fg: RGB, bg: RGB, order: i32, dmg : i32) 
        -> ProjectileRequest
    {
        ProjectileRequest{frame_time,path,projectile_type,glyph,fg,bg,order,dmg}
    }
}

pub struct ProjectileBuilder
{
    requests : Vec<ProjectileRequest>
}

impl ProjectileBuilder
{
    pub fn new() -> ProjectileBuilder
    {
        ProjectileBuilder{requests: Vec::new()}
    }

    pub fn add_request(&mut self, frame_time: f32, path: Vec<Point>,
        projectile_type: ProjectileType, glyph: char, fg: RGB, bg: RGB, order: i32, dmg : i32)
    {
        self.requests.push(ProjectileRequest::new(frame_time, path, projectile_type, glyph, fg, bg, order, dmg));
    }
}

