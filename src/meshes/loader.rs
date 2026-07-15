use glam::{Mat3, Quat, Vec3};
use gltf::{self, Error};

use crate::meshes::{mesh::AABB, mesh_data::MeshData};

// Carga de meshes complejos/custom en formatos estandarizados
// Se "aplana" todos los submeshes para respetar estructura MeshData
// Algunos meshes NO tienen tanto texcoords ni vectores normales
// Se genera una provicional
pub fn load_mesh(path: &str, correction: Option<glam::Quat>) -> Result<MeshData, Error> {
    let (document, buffers, _) = gltf::import(path)?;

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut texcoords: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for (i, mesh) in document.meshes().enumerate() {
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
        // if path == "assets/raw_meshes/SailShip.glb" && i == 3 {
        //     break;
        // }
    }

    if normals.is_empty() {
        normals = generate_normals(&positions, &indices);
    }

    if texcoords.is_empty() {
        texcoords = generate_triplanar_uv(&positions, &normals);
    }

    //Corregir rotaciones
    let correction = correction.unwrap_or(Quat::IDENTITY);
    let correction = Mat3::from_quat(correction);

    for p in &mut positions {
        *p = (correction * Vec3::from_array(*p)).to_array();
    }

    for n in &mut normals {
        *n = (correction * Vec3::from_array(*n)).normalize().to_array();
    }

    let positions = positions.leak();

    Ok(MeshData {
        positions,
        normals: normals.leak(),
        texcoords: texcoords.leak(),
        indices: indices.leak(),
        aabb: AABB::new(positions),
    })
}

fn generate_normals(positions: &[[f32; 3]], indices: &[u32]) -> Vec<[f32; 3]> {
    let mut normals = vec![Vec3::ZERO; positions.len()];

    for triangle in indices.chunks_exact(3) {
        let i0 = triangle[0] as usize;
        let i1 = triangle[1] as usize;
        let i2 = triangle[2] as usize;

        let v0 = Vec3::from(positions[i0]);
        let v1 = Vec3::from(positions[i1]);
        let v2 = Vec3::from(positions[i2]);

        let normal = (v1 - v0).cross(v2 - v0).normalize();

        normals[i0] += normal;
        normals[i1] += normal;
        normals[i2] += normal;
    }

    normals.into_iter().map(|n| n.normalize().into()).collect()
}

pub fn generate_triplanar_uv(positions: &[[f32; 3]], normals: &[[f32; 3]]) -> Vec<[f32; 2]> {
    let mut min = Vec3::splat(f32::INFINITY);
    let mut max = Vec3::splat(f32::NEG_INFINITY);

    for p in positions {
        let pos = Vec3::from(*p);

        min = min.min(pos);
        max = max.max(pos);
    }

    let size = max - min;

    let mut texcoords = Vec::with_capacity(positions.len());

    for (pos, normal) in positions.iter().zip(normals.iter()) {
        let p = Vec3::from(*pos);
        let n = Vec3::from(*normal).abs();

        let (u, v);

        if n.x >= n.y && n.x >= n.z {
            u = (p.z - min.z) / size.z.max(f32::EPSILON);
            v = (p.y - min.y) / size.y.max(f32::EPSILON);
        } else if n.y >= n.x && n.y >= n.z {
            u = (p.x - min.x) / size.x.max(f32::EPSILON);
            v = (p.z - min.z) / size.z.max(f32::EPSILON);
        } else {
            u = (p.x - min.x) / size.x.max(f32::EPSILON);
            v = (p.y - min.y) / size.y.max(f32::EPSILON);
        }

        texcoords.push([u, v]);
    }

    texcoords
}
