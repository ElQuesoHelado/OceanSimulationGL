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
        let waves: Vec<Wave> = vec![
            Wave {
                amplitude: 0.6f32,
                frequency: 0.08f32,
                direction: 0.0f32,
                phase: 0.0f32,
            },
            Wave {
                amplitude: 0.35f32,
                frequency: 0.12f32,
                direction: 0.4f32,
                phase: 1.3f32,
            },
        ];

        Self {
            time: 0f32,
            instance_id: 0,
            waves,
        }
    }
}
