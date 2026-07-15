use crate::meshes::mesh_data;
use crate::meshes::mesh_data::MeshData;
use glam::{Vec3, vec3};
use glow::HasContext;

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
    Boat,
    EmptyIsland,
    Grass,
    Island,
    PalmTree,
    SailBoat,
    Ship,
}

// Bebita de todos los meshes base
pub struct MeshLibrary {
    meshes: Vec<Mesh>,
}

impl MeshLibrary {
    pub fn new(gl: &glow::Context) -> Self {
        let meshes: Vec<Mesh> = vec![
            Mesh::upload(gl, mesh_data::cube()),
            Mesh::upload(gl, mesh_data::cone()),
            Mesh::upload(gl, mesh_data::cylinder()),
            Mesh::upload(gl, mesh_data::klein()),
            Mesh::upload(gl, mesh_data::pen()),
            Mesh::upload(gl, mesh_data::rock()),
            Mesh::upload(gl, mesh_data::sphere()),
            Mesh::upload(gl, mesh_data::tetrahedron()),
            Mesh::upload(gl, mesh_data::torus()),
            Mesh::upload(gl, mesh_data::billboard()),
            Mesh::upload(gl, mesh_data::plane(400)),
            Mesh::upload(gl, mesh_data::boat()),
            Mesh::upload(gl, mesh_data::empty_island()),
            Mesh::upload(gl, mesh_data::grass()),
            Mesh::upload(gl, mesh_data::island()),
            Mesh::upload(gl, mesh_data::palm_tree()),
            Mesh::upload(gl, mesh_data::sail_boat()),
            Mesh::upload(gl, mesh_data::ship()),
        ];

        Self { meshes }
    }

    pub fn add(&mut self, gl: &glow::Context, data: MeshData) {
        self.meshes.push(Mesh::upload(gl, data));
    }

    pub fn get(&self, id: MeshId) -> Option<&Mesh> {
        self.meshes.get(id as usize)
    }
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

// Mesh cargado
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

            // println!("{:?}\n\n", data.indices);

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
        // println!("{}", self.index_count);
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

pub struct SimpleMesh {
    vao: glow::VertexArray,
    vbo: glow::Buffer,
    vertex_count: i32,
    draw_mode: u32,
}

impl SimpleMesh {
    pub fn upload(gl: &glow::Context, positions: &[[f32; 3]], draw_mode: u32) -> Self {
        unsafe {
            let vao = gl.create_vertex_array().unwrap();
            let vbo = gl.create_buffer().unwrap();

            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.bind_vertex_array(None);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);

            gl.named_buffer_data_u8_slice(vbo, bytemuck::cast_slice(positions), glow::STATIC_DRAW);

            gl.vertex_array_vertex_buffer(vao, 0, Some(vbo), 0, 3 * 4);
            gl.enable_vertex_array_attrib(vao, 0);
            gl.vertex_array_attrib_format_f32(vao, 0, 3, glow::FLOAT, false, 0);
            gl.vertex_array_attrib_binding_f32(vao, 0, 0);

            Self {
                vao,
                vbo,
                vertex_count: positions.len() as i32,
                draw_mode,
            }
        }
    }

    pub fn draw(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_vertex_array(Some(self.vao));
            gl.draw_arrays(self.draw_mode, 0, self.vertex_count);
        }
    }

    pub fn destroy(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_vertex_array(self.vao);
            gl.delete_buffer(self.vbo);
        }
    }
}
