pub mod ocean;
pub mod rain;

use crate::{
    meshes::mesh::MeshLibrary,
    scene::Scene,
    simulations::{ocean::Ocean, rain::Rain},
    texture::TextureLibrary,
};

pub struct Simulation {
    pub rain: Rain,
    pub ocean: Ocean,
}

impl Simulation {
    pub fn new(
        scene: &mut Scene,
        texture_library: &TextureLibrary,
        mesh_library: &MeshLibrary,
    ) -> Self {
        Self {
            rain: Rain::new(scene, texture_library, mesh_library, 10),
            ocean: Ocean::new(scene, texture_library, mesh_library),
        }
    }

    pub fn update() {}
    pub fn draw() {}
}
