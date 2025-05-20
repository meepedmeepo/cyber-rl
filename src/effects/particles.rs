use bracket_lib::{color::RGB, prelude::FontCharType};

use crate::State;

#[derive(Clone, PartialEq)]
pub struct Particle {
    pub glyph: String,
    pub fg: RGB,
    pub bg: RGB,
    pub lifetime: f32,
}

#[derive(Clone, PartialEq)]
pub struct ParticleBurst {
    pub particle: Particle,
}

#[derive(Clone, PartialEq)]
pub struct ParticleLine {
    pub particle: Particle,
}

#[derive(Clone, PartialEq)]
pub struct ParticleAnimation {
    pub particle: Particle,
}

pub fn spawn_particle(
    state: &mut State,
    glyph: String,
    fg: RGB,
    bg: RGB,
    lifetime: f32,
    tile_idx: i32,
) {
    let x = tile_idx % state.map.map_width;
    let y = tile_idx / state.map.map_width;
    state
        .particle_builder
        .request(x, y, fg, bg, glyph, lifetime, None);
}
