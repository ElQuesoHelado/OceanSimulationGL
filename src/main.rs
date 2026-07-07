mod app;
mod camera;
mod ejemplo;
mod figures;
mod gizmo;
mod light;
mod mesh;
mod mops;
mod renderer;
mod scene;
mod shader;
mod texture;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run()
}
