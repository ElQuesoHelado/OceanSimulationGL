use glam::{Vec3, vec3};
use glow::HasContext;

use crate::mesh_data;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum MeshId {
    Cube,
    Cone,
    Cylinder,
    Klein,
    Pen,
    Rock,
    Sphere,
    Tetrahedron,
    Torus,
    Billboard,
    Plane,
}

// Duenia de todos los meshes base
pub struct MeshLibrary {
    meshes: Vec<Mesh>,
}