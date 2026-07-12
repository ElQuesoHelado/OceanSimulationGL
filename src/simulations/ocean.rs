use super::*;

struct Wave {
    amplitude: f32,
    frequency: f32,
    direction: f32,
    phase: f32,
}

pub struct Ocean {
    time: f32,
    // npoints: usize,
    instance_id: usize,
    waves: Vec<Wave>,
}

impl Ocean {
    pub fn new(scene: &mut Scene, texture_library: &TextureLibrary) -> Self {
        let waves: Vec<Wave> = vec![];

        Self {
            time: 0f32,
            instance_id: 0,
            waves,
        }
    }
}
