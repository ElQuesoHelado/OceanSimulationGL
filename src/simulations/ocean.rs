use std::f32::consts::PI;

use glam::Vec3;

use crate::scene::Instance;

use super::*;

pub struct Wave {
    pub amplitude: f32,
    pub frequency: f32,
    pub direction: f32,
    pub phase: f32,
}

pub struct Ocean {
    pub waves: Vec<Wave>,
}

fn wave_height_and_normal(x: f32, z: f32, time: f32, waves: &[Wave]) -> (f32, Vec3) {
    let mut height = 0.0;
    let mut d_h_dx = 0.0;
    let mut d_h_dz = 0.0;

    for w in waves {
        let k = (4.0 * PI * PI * w.frequency * w.frequency) / 9.81;
        let dir_cos = w.direction.cos();
        let dir_sin = w.direction.sin();
        let theta = k * (x * dir_cos + z * dir_sin) - 2.0 * PI * w.frequency * time + w.phase;

        let c = theta.cos();
        let s = theta.sin();
        height += w.amplitude * c;

        let d_theta = -w.amplitude * s * k;
        d_h_dx += d_theta * dir_cos;
        d_h_dz += d_theta * dir_sin;
    }

    let height = height * 8.0;
    let normal = Vec3::new(-d_h_dx, 1.0, -d_h_dz).normalize();

    (height, normal)
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

        Self { waves }
    }

    pub fn update(&mut self, instances: &mut [Instance], time: f32) {
        for inst in instances {
            let trans = &mut inst.transform;
            let (height, normal) =
                wave_height_and_normal(trans.get_x(), trans.get_z(), time, &self.waves);
            inst.transform.set_pos_y(height);
        }
    }
}
