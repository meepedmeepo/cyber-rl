use bracket_lib::{color::RGB, prelude::FontCharType};

use crate::{State};

#[derive(Clone, Copy, PartialEq)]
pub struct Particle
{
    pub glyph : FontCharType,
    pub fg : RGB,
    pub bg : RGB,
    pub lifetime : f32,
}
#[derive(Clone, Copy, PartialEq)]
pub struct ParticleBurst
{
    pub particle : Particle,
}
#[derive(Clone, Copy, PartialEq)]
pub struct ParticleLine
{
    pub particle : Particle,
}

#[derive(Clone, Copy, PartialEq)]
pub struct ParticleAnimation
{
    pub particle : Particle,
}

pub fn spawn_particle( state : &mut State, glyph : FontCharType, fg : RGB, bg : RGB, lifetime : f32, tile_idx : i32)
{
    let x = tile_idx % state.map.map_width;
    let y = tile_idx / state.map.map_width;
    state.particle_builder.request(x, y, fg, bg, glyph, lifetime, None);
}