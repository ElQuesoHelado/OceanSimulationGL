use glow::HasContext;

use crate::figures;

#[derive(Clone, Copy)]
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
}

// Duenia de todos los meshes base
pub struct MeshLibrary {
    meshes: Vec<Mesh>,
}

impl MeshLibrary {
    pub fn new(gl: &glow::Context) -> Self {
        let meshes: Vec<Mesh> = vec![
            Mesh::upload(gl, figures::cube()),
            Mesh::upload(gl, figures::cone()),
            Mesh::upload(gl, figures::cylinder()),
            Mesh::upload(gl, figures::klein()),
            Mesh::upload(gl, figures::pen()),
            Mesh::upload(gl, figures::rock()),
            Mesh::upload(gl, figures::sphere()),
            Mesh::upload(gl, figures::tetrahedron()),
            Mesh::upload(gl, figures::torus()),
            Mesh::upload(gl, figures::billboard()),
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

// Datos crudos de un mesh(Cube, Sphere, ...)
pub struct MeshData {
    pub positions: &'static [[f32; 3]],
    pub normals: &'static [[f32; 3]],
    pub texcoords: &'static [[f32; 2]],
    pub indices: &'static [u32],
}

// Mesh cargado
pub struct Mesh {
    vao: glow::VertexArray,
    vbo_positions: glow::Buffer,
    vbo_normals: glow::Buffer,
    vbo_texcoords: glow::Buffer,
    ebo: glow::Buffer,
    index_count: i32,
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
