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

pub struct AABB {
    pub min_point: glam::Vec3,
    pub max_point: glam::Vec3,
}

impl AABB {
    pub fn new(positions: &[[f32; 3]]) -> Self {
        let Some(first) = positions.first() else {
            return Self {
                min_point: vec3(0.0, 0.0, 0.0),
                max_point: vec3(0.0, 0.0, 0.0),
            };
        };

        let first = Vec3::from_slice(first);

        let (min_point, max_point) =
            positions
                .iter()
                .fold((first, first), |(min, max), &[x, y, z]| {
                    let p = vec3(x, y, z);
                    (min.min(p), max.max(p))
                });

        Self {
            min_point,
            max_point,
        }
    }
}

// Datos crudos de un mesh (Cubo, Esfera, ...)
pub struct MeshData {
    pub positions: &'static [[f32; 3]],
    pub normals: &'static [[f32; 3]],
    pub texcoords: &'static [[f32; 2]],
    pub indices: &'static [u32],
    pub aabb: AABB,
}

// Mesh ya cargado en la GPU
pub struct Mesh {
    vao: glow::VertexArray,
    vbo_positions: glow::Buffer,
    vbo_normals: glow::Buffer,
    vbo_texcoords: glow::Buffer,
    ebo: glow::Buffer,
    index_count: i32,
    pub data: MeshData,
}