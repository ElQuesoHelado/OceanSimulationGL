mod app;
mod camera;
mod gizmo;
mod input_state;
mod light;
mod material;
mod meshes;
mod mops;
mod renderer;
mod scene;
mod shader;
mod simulations;
mod texture;

// use crate::app;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run()
}
