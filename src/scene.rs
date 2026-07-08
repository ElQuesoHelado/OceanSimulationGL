use glam::vec4;

use crate::{material::Material, mesh::MeshId, mops::Transform, texture::TextureLibrary};

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
    pub prim_instances: Vec<Instance>,
    pub bill_instances: Vec<Instance>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            prim_instances: Vec::new(),
            bill_instances: Vec::new(),
        }
    }

    pub fn add_prim_instance(&mut self, instance: Instance) {
        self.prim_instances.push(instance);
    }

    pub fn add_bill_instance(&mut self, texture_library: &TextureLibrary) {
        let material = Material::new(texture_library, vec4(1., 1., 1., 1.), 32., "blank")
            .expect("Textura no encontrada");

        self.bill_instances.push(Instance {
            transform: Transform::new(),
            mesh_id: MeshId::Billboard,
            material,
        });
    }

    pub fn add_bill_instance_trans(
        &mut self,
        transform: Transform,
        texture_library: &TextureLibrary,
    ) -> usize {
        let material = Material::new(texture_library, vec4(1., 1., 1., 1.), 32., "blank")
            .expect("Textura no encontrada");

        self.bill_instances.push(Instance {
            transform,
            mesh_id: MeshId::Billboard,
            material,
        });

        self.bill_instances.len() - 1
    }

    pub fn bill_instance_mut(&mut self, id: usize) -> &mut Instance {
        self.bill_instances.get_mut(id).expect("Id no existe")
    }
}
