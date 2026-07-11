mod app;
mod camera;
mod gizmo;
mod input_state;
mod light;
mod material;
mod mesh;
mod mesh_data;
mod mops;
mod rain;
mod renderer;
mod scene;
mod shader;
mod texture;

// use crate::app;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run()
}
