// TODO: cambio a "particulas" genericas, en base a fuerza
// - Check con delets de figuras

use glam::{vec3, vec4};
use rand::RngExt;

use crate::{
    material::Material,
    meshes::mesh::{MeshHandle, MeshLibrary},
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
    pub fn new(
        scene: &mut Scene,
        texture_library: &TextureLibrary,
        mesh_library: &MeshLibrary,
        n_drops: usize,
    ) -> Self {
        let mut rng = rand::rng();
        let mut drops = Vec::with_capacity(n_drops);

        let material = Material::new(texture_library, vec4(1., 1., 1., 1.), 32., "blank")
            .expect("Textura no encontrada");

        let mesh_handle = mesh_library.get_handle_from_name("billboard").unwrap();

        for _ in 0..n_drops {
            let transform = *Transform::new()
                .translate(vec3(
                    rng.random_range(-50.0..50.0),
                    rng.random_range(2.0..30.0),
                    rng.random_range(-50.0..50.0),
                ))
                .scale(vec3(0.1, 0.8, 1.0));

            let id = scene.add_billboard_instance(Instance {
                transform,
                mesh_handle,
                material,
            });

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
            let instance = match scene.get_billboard_instance_mut(drop.instance_id) {
                Some(instance) => instance,
                None => {
                    continue;
                }
            };

            instance
                .transform
                .translate(vec3(0.0, -drop.speed * dt, 0.0));

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
