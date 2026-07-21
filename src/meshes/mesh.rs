use std::collections::HashMap;
use std::path::Path;

use crate::meshes::loader::{load_mesh, load_obj_mesh};
use crate::meshes::mesh_data::MeshData;
use crate::scene::Instance;
use crate::{material::Material, meshes::mesh_data};
use glam::{Vec3, vec3};
use glow::HasContext;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum MeshKind {
    Standard,
    Billboard,
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct MeshHandle {
    pub id: usize,
    pub kind: MeshKind,
}

fn add_inner(
    gl: &glow::Context,
    meshes: &mut Vec<Mesh>,
    names: &mut HashMap<String, MeshHandle>,
    data: MeshData,
    name: &str,
    kind: MeshKind,
) {
    meshes.push(Mesh::upload(gl, data));
    names.insert(
        name.to_string(),
        MeshHandle {
            id: names.len(),
            kind,
        },
    );
}

pub struct MeshLibrary {
    meshes: Vec<Mesh>,
    pub names: HashMap<String, MeshHandle>,
}

impl MeshLibrary {
    pub fn new(gl: &glow::Context) -> Self {
        let mut meshes: Vec<Mesh> = Vec::new();
        let mut names: HashMap<String, MeshHandle> = HashMap::new();

        add_inner(
            gl,
            &mut meshes,
            &mut names,
            mesh_data::cube(),
            "cube",
            MeshKind::Standard,
        );
        add_inner(
            gl,
            &mut meshes,
            &mut names,
            mesh_data::cone(),
            "cone",
            MeshKind::Standard,
        );
        add_inner(
            gl,
            &mut meshes,
            &mut names,
            mesh_data::cylinder(),
            "cylinder",
            MeshKind::Standard,
        );
        add_inner(
            gl,
            &mut meshes,
            &mut names,
            mesh_data::klein(),
            "klein",
            MeshKind::Standard,
        );
        add_inner(
            gl,
            &mut meshes,
            &mut names,
            mesh_data::pen(),
            "pen",
            MeshKind::Standard,
        );
        add_inner(
            gl,
            &mut meshes,
            &mut names,
            mesh_data::rock(),
            "rock",
            MeshKind::Standard,
        );
        add_inner(
            gl,
            &mut meshes,
            &mut names,
            mesh_data::sphere(),
            "sphere",
            MeshKind::Standard,
        );
        add_inner(
            gl,
            &mut meshes,
            &mut names,
            mesh_data::tetrahedron(),
            "tetrahedron",
            MeshKind::Standard,
        );
        add_inner(
            gl,
            &mut meshes,
            &mut names,
            mesh_data::torus(),
            "torus",
            MeshKind::Standard,
        );
        add_inner(
            gl,
            &mut meshes,
            &mut names,
            mesh_data::billboard(),
            "billboard",
            MeshKind::Billboard,
        );
        add_inner(
            gl,
            &mut meshes,
            &mut names,
            mesh_data::plane(400),
            "plane",
            MeshKind::Standard,
        );
        add_inner(
            gl,
            &mut meshes,
            &mut names,
            mesh_data::boat(),
            "boat",
            MeshKind::Standard,
        );
        add_inner(
            gl,
            &mut meshes,
            &mut names,
            mesh_data::empty_island(),
            "empty_island",
            MeshKind::Standard,
        );
        add_inner(
            gl,
            &mut meshes,
            &mut names,
            mesh_data::grass(),
            "grass",
            MeshKind::Standard,
        );
        add_inner(
            gl,
            &mut meshes,
            &mut names,
            mesh_data::island(),
            "island",
            MeshKind::Standard,
        );
        add_inner(
            gl,
            &mut meshes,
            &mut names,
            mesh_data::palm_tree(),
            "palm_tree",
            MeshKind::Standard,
        );
        add_inner(
            gl,
            &mut meshes,
            &mut names,
            mesh_data::sail_boat(),
            "sail_boat",
            MeshKind::Standard,
        );
        add_inner(
            gl,
            &mut meshes,
            &mut names,
            mesh_data::ship(),
            "ship",
            MeshKind::Standard,
        );

        Self { meshes, names }
    }

    pub fn add_from_file<P: AsRef<Path>>(&mut self, gl: &glow::Context, path: P) {
        let path = path.as_ref();

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());

        let data = match ext.as_deref() {
            Some("glb") | Some("gltf") => load_mesh(path, None),
            Some("obj") => load_obj_mesh(path, None),
            _ => {
                println!("Extension de mesh no soportada");
                return;
            }
        };

        let Ok(data) = data else {
            println!("Mesh Invalido");
            return;
        };

        let Some(name) = path.file_stem() else {
            println!("Nombre Mesh Invalido");
            return;
        };

        let name = &name.to_string_lossy().into_owned().to_lowercase();

        add_inner(
            gl,
            &mut self.meshes,
            &mut self.names,
            data,
            name,
            MeshKind::Standard,
        );
    }

    pub fn get_handle_from_name(&self, name: &str) -> Option<MeshHandle> {
        self.names.get(name).copied()
    }

    pub fn get_from_name(&self, name: &str) -> Option<&Mesh> {
        self.get(*self.names.get(name)?)
    }

    pub fn get(&self, handle: MeshHandle) -> Option<&Mesh> {
        self.meshes.get(handle.id)
    }

    pub fn instantiate_from_name(
        &self,
        name: &str,
        material: &Material,
        scale: Option<Vec3>,
        rotate: Option<Vec3>,
    ) -> Option<Instance> {
        if let Some(handle) = self.get_handle_from_name(name) {
            return self.instantiate_from_handle(handle, material, scale, rotate);
        };

        None
    }

    pub fn instantiate_from_handle(
        &self,
        mesh_handle: MeshHandle,
        material: &Material,
        scale: Option<Vec3>,
        rotate: Option<Vec3>,
    ) -> Option<Instance> {
        let mesh = self.get(mesh_handle)?;

        let mut transform = mesh.data.correction;

        if let Some(scale) = scale {
            transform.scale(scale);
        }

        if let Some(rotate) = rotate {
            transform
                .rotate_x(rotate.x)
                .rotate_y(rotate.y)
                .rotate_z(rotate.z);
        }

        Some(Instance {
            mesh_handle,
            transform,
            material: *material,
        })
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
