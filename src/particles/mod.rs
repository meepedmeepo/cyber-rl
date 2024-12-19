use bracket_lib::color::RGB;
pub mod particle_system;


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
    lifetime: f32
}

pub struct ParticleBuilder
{
    requests : Vec<ParticleRequest>
}

impl ParticleBuilder
{
    pub fn new() -> ParticleBuilder
    {
        ParticleBuilder{requests : Vec::new()}
    }

    pub fn request(&mut self, x:i32, y : i32, fg : RGB, bg : RGB, glyph : char, lifetime : f32)
    {
        self.requests.push(ParticleRequest {x,y,fg, bg, glyph, lifetime});
    }
}