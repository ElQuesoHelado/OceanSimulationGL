use crate::{material::Material, mesh::MeshId, mops::Transform};

pub struct Instance {
    pub transform: Transform,
    pub mesh: MeshId,
    pub material: Material,
}

impl Instance {
    pub fn new(mesh: MeshId, material: Material) -> Self {
        Self {
            transform: Transform::new(),
            mesh,
            material,
        }
    }
}

pub struct Scene {
    pub prim_instances: Vec<Instance>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            prim_instances: Vec::new(),
        }
    }

    pub fn add_prim_instance(&mut self, instance: Instance) {
        self.prim_instances.push(instance);
    }
}
