use std::f32::consts::PI;

use glam::{Quat, Vec3, Vec4, vec3, vec4};

use crate::{
    material::Material,
    meshes::mesh::{MeshHandle, MeshLibrary},
    scene::{Instance, Scene},
    texture::TextureLibrary,
};

pub struct Wave {
    pub amplitude: f32,
    pub frequency: f32,
    pub direction: f32,
    pub phase: f32,
}

pub struct Ocean {
    pub waves: Vec<Wave>,
    pub inst_idx_start: usize, // Inicio de indices a simular
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
    pub fn new(
        scene: &mut Scene,
        texture_library: &TextureLibrary,
        mesh_library: &MeshLibrary,
    ) -> Self {
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

        let inst_idx_start = setup(scene, texture_library, mesh_library);

        Self {
            waves,
            inst_idx_start,
        }
    }

    pub fn update(&mut self, instances: &mut [Instance], time: f32) {
        for inst in &mut instances[self.inst_idx_start..] {
            let trans = &mut inst.transform;
            let (height, normal) =
                wave_height_and_normal(trans.get_x(), trans.get_z(), time, &self.waves);
            inst.transform.set_pos_y(height);
            inst.transform
                .set_rotation(Quat::from_axis_angle(normal, PI));
        }
    }
}

fn setup(scene: &mut Scene, texture_library: &TextureLibrary, mesh_library: &MeshLibrary) -> usize {
    let sun_material = Material::new(
        texture_library,
        vec4(255f32, 255f32, 50f32, 1f32),
        200f32,
        "blank",
    )
    .expect("Error cargando textura");

    let wood_material = Material::new(texture_library, Vec4::ONE, 32f32, "roof_wood")
        .expect("Error cargando textura");

    let green_material = Material::new(texture_library, vec4(0.5, 0.8, 0.5, 1f32), 3000f32, "tree")
        .expect("Error cargando textura");

    let sand_material = Material::new(texture_library, vec4(1.0, 0.7, 0.55, 1f32), 32f32, "blank")
        .expect("Error cargando textura");

    // let mut island1 = mesh_library.instantiate_from_name(
    //     "island",
    //     &green_material,
    //     Some(vec3(0.7f32, 0.7f32, 0.7f32)),
    //     None,
    // );

    let mut island1 = mesh_library
        .instantiate_from_name(
            "island",
            &green_material,
            //Some(vec3(0.7f32, 0.7f32, 0.7f32)),
            None,
            None,
        )
        .expect("Instancia no creada simulacion ocean");
    island1.transform.set_position(vec3(100f32, 5f32, 50f32));
    island1.transform.scale(vec3(4f32, 4f32, 4f32));

    let mut island2 = mesh_library
        .instantiate_from_name(
            "empty_island",
            &sand_material,
            //Some(vec3(75f32, 200f32, 75f32)),
            None,
            None,
        )
        .expect("Instancia no creada simulacion ocean");
    island2.transform.set_position(vec3(150f32, 5f32, 200f32));
    island2.transform.scale(vec3(4f32, 7f32, 4f32));

    let mut palm1 = mesh_library
        .instantiate_from_name(
            "palm_tree",
            &green_material,
            //Some(vec3(15f32, 15f32, 15f32)),
            None,
            None,
        )
        .expect("Instancia no creada simulacion ocean");
    palm1.transform.set_position(vec3(150f32, 13f32, 200f32));

    let mut palm2 = mesh_library
        .instantiate_from_name(
            "palm_tree",
            &green_material,
            //Some(vec3(15f32, 15f32, 15f32)),
            None,
            None,
        )
        .expect("Instancia no creada simulacion ocean");
    palm2.transform.set_position(vec3(180f32, 10f32, 200f32));

    let mut sun = mesh_library
        .instantiate_from_name(
            "billboard",
            &sun_material,
            //Some(vec3(15f32, 15f32, 15f32)),
            None,
            None,
        )
        .expect("Instancia no creada simulacion ocean");
    sun.transform.set_position(vec3(-200f32, 500f32, -200f32));

    scene.add_normal_instance(island1);
    scene.add_normal_instance(island2);
    scene.add_normal_instance(palm1);
    scene.add_normal_instance(palm2);
    scene.add_billboard_instance(sun);

    scene.normal_instances.len()
}
