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

impl Mesh {
    pub fn upload(gl: &glow::Context, data: MeshData) -> Self {
        unsafe {
            let vao = gl.create_vertex_array().unwrap();
            let vbo_positions = gl.create_buffer().unwrap();
            let vbo_normals = gl.create_buffer().unwrap();
            let vbo_texcoords = gl.create_buffer().unwrap();
            let ebo = gl.create_buffer().unwrap();

            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo_positions));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo_normals));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo_texcoords));
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.bind_vertex_array(None);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);

            gl.named_buffer_data_u8_slice(
                vbo_positions,
                bytemuck::cast_slice(&data.positions),
                glow::STATIC_DRAW,
            );
            gl.named_buffer_data_u8_slice(
                vbo_normals,
                bytemuck::cast_slice(&data.normals),
                glow::STATIC_DRAW,
            );
            gl.named_buffer_data_u8_slice(
                vbo_texcoords,
                bytemuck::cast_slice(&data.texcoords),
                glow::STATIC_DRAW,
            );
            gl.named_buffer_data_u8_slice(
                ebo,
                bytemuck::cast_slice(&data.indices),
                glow::STATIC_DRAW,
            );

            // posiciones -> binding 0
            gl.vertex_array_vertex_buffer(vao, 0, Some(vbo_positions), 0, 3 * 4);
            gl.enable_vertex_array_attrib(vao, 0);
            gl.vertex_array_attrib_format_f32(vao, 0, 3, glow::FLOAT, false, 0);
            gl.vertex_array_attrib_binding_f32(vao, 0, 0);

            // normales -> binding 1
            gl.vertex_array_vertex_buffer(vao, 1, Some(vbo_normals), 0, 3 * 4);
            gl.enable_vertex_array_attrib(vao, 1);
            gl.vertex_array_attrib_format_f32(vao, 1, 3, glow::FLOAT, false, 0);
            gl.vertex_array_attrib_binding_f32(vao, 1, 1);

            // uv -> binding 2
            gl.vertex_array_vertex_buffer(vao, 2, Some(vbo_texcoords), 0, 2 * 4);
            gl.enable_vertex_array_attrib(vao, 2);
            gl.vertex_array_attrib_format_f32(vao, 2, 2, glow::FLOAT, false, 0);
            gl.vertex_array_attrib_binding_f32(vao, 2, 2);

            gl.vertex_array_element_buffer(vao, Some(ebo));
            Self {
                vao,
                vbo_positions,
                vbo_normals,
                vbo_texcoords,
                ebo,
                index_count: data.indices.len() as i32,
                data,
            }
        }
    }
    
    pub fn draw(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_vertex_array(Some(self.vao));
            gl.draw_elements(glow::TRIANGLES, self.index_count, glow::UNSIGNED_INT, 0);
        }
    }

    pub fn destroy(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_vertex_array(self.vao);
            gl.delete_buffer(self.vbo_positions);
            gl.delete_buffer(self.vbo_normals);
            gl.delete_buffer(self.vbo_texcoords);
            gl.delete_buffer(self.ebo);
        }
    }
}
