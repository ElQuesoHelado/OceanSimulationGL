use gltf::{self, Error};

use crate::meshes::{mesh::AABB, mesh_data::MeshData};

// Carga de meshes complejos/custom en formatos estandarizados
// Se "aplana" todos los submeshes para respetar estructura MeshData
// Algunos meshes NO tienen tanto texcoords ni vectores normales
pub fn load_mesh(path: &str) -> Result<MeshData, Error> {
    // pub fn load_mesh(path: &str) {
    let (document, buffers, _) = gltf::import(path)?;

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut texcoords: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for mesh in document.meshes() {
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

            let pos: Vec<[f32; 3]> = reader.read_positions().unwrap().collect();

            let nor: Vec<[f32; 3]> = match reader.read_normals() {
                Some(v) => v.collect(),
                None => Vec::new(),
            };

            let uv: Vec<[f32; 2]> = match reader.read_tex_coords(0) {
                Some(v) => v.into_f32().collect(),
                None => Vec::new(),
            };

            let idx: Vec<u32> = reader.read_indices().unwrap().into_u32().collect();

            let base = positions.len() as u32;

            positions.extend(pos);
            normals.extend(nor);
            texcoords.extend(uv);

            indices.extend(idx.into_iter().map(|i| i + base));
        }
    }

    let positions = positions.leak();

    println!(
        "pos: {}, normls: {}, texcoords: {}, indices: {}",
        positions.len(),
        normals.len(),
        texcoords.len(),
        indices.len()
    );

    let mut min = positions[0];
    let mut max = positions[0];

    for p in &positions[1..] {
        for i in 0..3 {
            min[i] = min[i].min(p[i]);
            max[i] = max[i].max(p[i]);
        }
    }

    println!("x: min = {}, max = {}", min[0], max[0]);
    println!("y: min = {}, max = {}", min[1], max[1]);
    println!("z: min = {}, max = {}", min[2], max[2]);

    Ok(MeshData {
        positions,
        normals: normals.leak(),
        texcoords: texcoords.leak(),
        indices: indices.leak(),
        aabb: AABB::new(positions),
    })
}
