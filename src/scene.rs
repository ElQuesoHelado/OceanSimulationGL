use glam::vec4;

use crate::{material::Material, mesh::MeshId, mops::Transform, texture::TextureLibrary};

#[derive(Clone)]
pub struct Instance {
    pub transform: Transform,
    pub mesh_id: MeshId,
    pub material: Material,
}

impl Instance {
    pub fn new(mesh_id: MeshId, material: Material) -> Self {
        Self {
            transform: Transform::new(),
            mesh_id,
            material,
        }
    }
}

pub struct Scene {
    pub normal_instances: Vec<Instance>,
    pub billboard_instances: Vec<Instance>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            normal_instances: Vec::new(),
            billboard_instances: Vec::new(),
        }
    }

    pub fn add_normal_instance(&mut self, instance: Instance) -> usize {
        self.normal_instances.push(instance);
        self.normal_instances.len() - 1
    }

    pub fn add_billboard_instance(&mut self, instance: Instance) -> usize {
        self.billboard_instances.push(instance);
        self.billboard_instances.len() - 1
    }

    // pub fn add_instance(&mut self, texture_library: &TextureLibrary, texture_name: &str) {
    //     let material = Material::new(texture_library, vec4(1., 1., 1., 1.), 32., tex)
    //         .expect("Textura no encontrada");
    //
    //     self.bill_instances.push(Instance {
    //         transform: Transform::new(),
    //         mesh_id: MeshId::Billboard,
    //         material,
    //     });
    // }

    // pub fn add_bill_instance_transformed(
    //     &mut self,
    //     transform: Transform,
    //     texture_library: &TextureLibrary,
    // ) -> usize {
    //     let material = Material::new(texture_library, vec4(1., 1., 1., 1.), 32., "blank")
    //         .expect("Textura no encontrada");
    //
    //     self.bill_instances.push(Instance {
    //         transform,
    //         mesh_id: MeshId::Billboard,
    //         material,
    //     });
    //
    //     self.bill_instances.len() - 1
    // }

    pub fn get_instance_mut(&mut self, id: usize) -> Option<&mut Instance> {
        self.normal_instances.get_mut(id)
    }
}
