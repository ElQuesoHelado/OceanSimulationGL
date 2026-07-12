mod ocean;
mod rain;

use crate::{scene::Scene, simulations::rain::Rain, texture::TextureLibrary};

pub struct Simulation {
    rain: Rain,
    //ocean: Ocean,
}

impl Simulation {
    pub fn new(scene: &mut Scene, texture_library: &TextureLibrary) -> Self {
        Self {
            rain: Rain::new(scene, texture_library, 10),
        }
    }

    pub fn update() {}
    pub fn draw() {}
}
