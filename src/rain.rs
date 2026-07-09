// TODO: cambio a "particulas" genericas, en base a fuerza
// - Check con delets de figuras

use glam::vec3;
use rand::RngExt;

use crate::{
    mops::Transform,
    scene::{Instance, Scene},
    texture::TextureLibrary,
};

pub struct RainDrop {
    instance_id: usize,
    speed: f32,
}

pub struct Rain {
    drops: Vec<RainDrop>,
}

impl Rain {
    pub fn new(scene: &mut Scene, texture_library: &TextureLibrary, n_drops: usize) -> Self {
        let mut rng = rand::rng();
        let mut drops = Vec::with_capacity(n_drops);

        for _ in 0..n_drops {
            let mut transform = Transform::new();

            transform
                .trans(vec3(
                    rng.random_range(-50.0..50.0),
                    rng.random_range(2.0..30.0),
                    rng.random_range(-50.0..50.0),
                ))
                .scale(vec3(0.1, 0.8, 1.0));

            //transform.trans(vec3(0.0, 0.0, 0.0));

            let id = scene.add_bill_instance_trans(transform, texture_library);

            drops.push(RainDrop {
                instance_id: id,
                speed: rng.random_range(6.0..12.0),
            });
        }

        Self { drops }
    }

    pub fn update(&mut self, scene: &mut Scene, dt: f32) {
        let mut rng = rand::rng();

        for drop in &mut self.drops {
            let instance = scene.bill_instance_mut(drop.instance_id);

            instance.transform.trans(vec3(0.0, -drop.speed * dt, 0.0));

            if instance.transform.position().y <= 0.0 {
                instance.transform.set_position(vec3(
                    rng.random_range(-50.0..50.0),
                    rng.random_range(2.0..30.0),
                    rng.random_range(-50.0..50.0),
                ));
            }
        }
    }
}
