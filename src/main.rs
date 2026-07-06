mod camera;
mod ejemplo;
mod figures;
mod instance;
mod mesh;
mod mops;
mod shader;
mod texture;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ejemplo::run()
}
