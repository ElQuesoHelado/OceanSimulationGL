mod camera;
mod ejemplo;
mod mops;
mod shader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ejemplo::run()
}
