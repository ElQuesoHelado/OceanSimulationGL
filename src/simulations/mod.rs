pub mod ocean;
pub mod rain;

use crate::{
    scene::Scene, simulations::ocean::Ocean, simulations::rain::Rain, texture::TextureLibrary,
};

pub struct Simulation {
    pub rain: Rain,
    pub ocean: Ocean,
}

impl Simulation {
    pub fn new(scene: &mut Scene, texture_library: &TextureLibrary) -> Self {
        Self {
            rain: Rain::new(scene, texture_library, 10),
            ocean : Ocean::new(scene, texture_library);
        }
    }

    pub fn update() {}
    pub fn draw() {}
}
