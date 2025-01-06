use bracket_lib::color::RGB;
use hecs::Entity;
pub mod particle_system;

pub struct Particle
{
    pub glyph : char,
    pub fg : RGB,
    pub bg : RGB,
    pub lifetime : f32
}
#[derive(Copy,Clone,  PartialEq,  PartialOrd)]
pub struct ParticleLifetime
{
    pub lifetime: f32,
}

pub struct ParticleRequest
{
    x : i32,
    y : i32,
    fg : RGB,
    bg : RGB,
    glyph : char,
    lifetime: f32,
    target: Option<Entity>
}

pub struct ParticleBuilder
{
    requests : Vec<ParticleRequest>
}

struct ParticleFollowEntity
{
    pub target : Entity,
}

impl ParticleBuilder
{
    pub fn new() -> ParticleBuilder
    {
        ParticleBuilder{requests : Vec::new()}
    }
    //TODO: all glyphs should be u16/u8 btw lmao
    pub fn request(&mut self, x:i32, y : i32, fg : RGB, bg : RGB, glyph : char,
        lifetime : f32, follow_target : Option<Entity>)
    {
        self.requests.push(ParticleRequest {x,y,fg, bg, glyph, lifetime, target : follow_target});
    }

}