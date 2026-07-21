use std::path::Path;

use glam::{Mat3, Quat, Vec3};
use gltf::{self};

use crate::{
    meshes::{mesh::AABB, mesh_data::MeshData},
    mops::Transform,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error cargando GLTF: {0}")]
    Gltf(#[from] gltf::Error),

    #[error("error cargando OBJ: {0}")]
    Obj(#[from] tobj::LoadError),

    #[error("mesh sin nombre válido")]
    InvalidName,
}

// Carga de meshes complejos/custom en formatos estandarizados
// Se "aplana" todos los submeshes para respetar estructura MeshData
// Algunos meshes NO tienen tanto texcoords ni vectores normales
// Se genera una provicional
pub fn load_mesh<P: AsRef<Path>>(
    path: P,
    correction: Option<glam::Quat>,
) -> Result<MeshData, Error> {
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

    Ok(MeshData::new(
        positions,
        normals.leak(),
        texcoords.leak(),
        indices.leak(),
    ))
}

pub fn load_obj_mesh<P: AsRef<Path>>(
    path: P,
    correction: Option<glam::Quat>,
) -> Result<MeshData, Error> {
    let load_opts = tobj::LoadOptions {
        single_index: true,
        triangulate: true,
        ..Default::default()
    };

    let (models, _materials) = tobj::load_obj(path.as_ref(), &load_opts)?;

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut texcoords: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for model in models {
        let mesh = model.mesh;

        let pos: Vec<[f32; 3]> = mesh
            .positions
            .chunks_exact(3)
            .map(|p| [p[0], p[1], p[2]])
            .collect();

        let nor: Vec<[f32; 3]> = mesh
            .normals
            .chunks_exact(3)
            .map(|n| [n[0], n[1], n[2]])
            .collect();

        let uv: Vec<[f32; 2]> = mesh
            .texcoords
            .chunks_exact(2)
            .map(|t| [t[0], t[1]])
            .collect();

        let idx: Vec<u32> = mesh.indices;

        let base = positions.len() as u32;

        positions.extend(pos);
        normals.extend(nor);
        texcoords.extend(uv);

        indices.extend(idx.into_iter().map(|i| i + base));
    }

    if normals.is_empty() {
        normals = generate_normals(&positions, &indices);
    }

    if texcoords.is_empty() {
        texcoords = generate_triplanar_uv(&positions, &normals);
    }

    let correction = correction.unwrap_or(Quat::IDENTITY);
    let correction = Mat3::from_quat(correction);

    for p in &mut positions {
        *p = (correction * Vec3::from_array(*p)).to_array();
    }

    for n in &mut normals {
        *n = (correction * Vec3::from_array(*n)).normalize().to_array();
    }

    let positions = positions.leak();

    Ok(MeshData::new(
        positions,
        normals.leak(),
        texcoords.leak(),
        indices.leak(),
    ))
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
