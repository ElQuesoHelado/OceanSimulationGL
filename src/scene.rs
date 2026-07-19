use std::f32::consts::PI;

use glam::{vec3, vec4};

use crate::{material::Material, meshes::mesh::MeshId, mops::Transform, texture::TextureLibrary};

#[derive(Clone)]
pub struct Instance {
    pub transform: Transform,
    pub mesh_id: MeshId,
    pub material: Material,
}

impl Instance {
    pub fn new(mesh_id: MeshId, material: Material) -> Self {
        let mut transform = Transform::new();
        match mesh_id {
            MeshId::Boat | MeshId::PalmTree => transform.scale(vec3(500f32, 500f32, 500f32)),
            MeshId::Ship => transform.scale(vec3(600f32, 600f32, 600f32)),
            // .rotate_x(-PI / 2f32),
            MeshId::SailBoat => transform
                .scale(vec3(3000f32, 3000f32, 3000f32))
                .rotate_x(-PI / 2f32),
            MeshId::Island => transform.scale(vec3(0.7f32, 0.7f32, 0.7f32)),
            MeshId::EmptyIsland => transform.scale(vec3(75f32, 200f32, 75f32)),
            MeshId::Grass => transform.scale(vec3(30f32, 10f32, 30f32)),
            _ => transform.scale(vec3(10f32, 10f32, 10f32)),
        };

        Self {
            transform,
            mesh_id,
            material,
        }
    }
}

#[derive(Clone)]
pub struct Skybox {
    pub transform: Transform,
    pub mesh_id: MeshId,
    pub cube_map_id: u32,
}

impl Skybox {
    pub fn new(texture_library: &TextureLibrary, texture_name: &str) -> Option<Self> {
        let transform = Transform::new();

        let cube_map_id = texture_library.get_id_from_name(texture_name)?;

        Some(Self {
            transform,
            mesh_id: MeshId::Cube,
            cube_map_id,
        })
    }
}

pub struct Scene {
    pub normal_instances: Vec<Instance>,
    pub billboard_instances: Vec<Instance>,
    pub ocean_instances: Vec<Instance>,
    pub skybox: Option<Skybox>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            normal_instances: Vec::new(),
            billboard_instances: Vec::new(),
            ocean_instances: Vec::new(),
            skybox: None,
        }
    }

    pub fn add_normal_instance(&mut self, instance: Instance) -> usize {
        self.normal_instances.push(instance);
        self.normal_instances.len() - 1
    }

    pub fn set_skybox_instance(&mut self, skybox: Option<Skybox>) {
        self.skybox = skybox;
    }

    pub fn add_ocean_instance(&mut self, instance: Instance) -> usize {
        self.ocean_instances.push(instance);
        self.ocean_instances.len() - 1
    }

    pub fn add_billboard_instance(&mut self, instance: Instance) -> usize {
        self.billboard_instances.push(instance);
        self.billboard_instances.len() - 1
    }

    pub fn get_instance_mut(&mut self, id: usize) -> Option<&mut Instance> {
        self.normal_instances.get_mut(id)
    }

    pub fn get_billboard_instance_mut(&mut self, id: usize) -> Option<&mut Instance> {
        self.billboard_instances.get_mut(id)
    }
}
