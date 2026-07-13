use super::*;

pub struct Wave {
    pub amplitude: f32,
    pub frequency: f32,
    pub direction: f32,
    pub phase: f32,
}

pub struct Ocean {
    pub time: f32,
    // npoints: usize,
    pub instance_id: usize, //Solo un oceano
    pub waves: Vec<Wave>,
}

impl Ocean {
    // pub fn new(scene: &mut Scene, texture_library: &TextureLibrary) -> Self {
    pub fn new() -> Self {
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
            Wave {
                amplitude: 0.18f32,
                frequency: 0.18f32,
                direction: -0.3f32,
                phase: 2.7f32,
            },
            Wave {
                amplitude: 0.09f32,
                frequency: 0.28f32,
                direction: 0.7f32,
                phase: 0.5f32,
            },
        ];

        Self {
            time: 0f32,
            instance_id: 0,
            waves,
        }
    }
}
