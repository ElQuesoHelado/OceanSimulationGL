use std::f32::consts::FRAC_PI_2;

use crate::{
    meshes::{loader::load_mesh, mesh::AABB},
    mops::Transform,
};
use bytemuck::cast_slice;
use glam::vec3;

// Datos crudos de un mesh(Cube, Sphere, ...)
pub struct MeshData {
    pub positions: &'static [[f32; 3]],
    pub normals: &'static [[f32; 3]],
    pub texcoords: &'static [[f32; 2]],
    pub indices: &'static [u32],
    pub aabb: AABB,
    pub correction: Transform,
}

impl MeshData {
    pub fn new(
        positions: &'static [[f32; 3]],
        normals: &'static [[f32; 3]],
        texcoords: &'static [[f32; 2]],
        indices: &'static [u32],
    ) -> Self {
        let aabb = AABB::new(positions);
        let scale_factor =
            (3f32 * 20f32.powi(2)).sqrt() / (aabb.max_point - aabb.min_point).length();

        MeshData {
            positions,
            normals,
            texcoords,
            indices,
            aabb,
            correction: *Transform::new().scale(vec3(scale_factor, scale_factor, scale_factor)),
        }
    }
}

#[repr(C)]
struct AlignedTo<Align, Bytes: ?Sized> {
    _align: [Align; 0],
    bytes: Bytes,
}

macro_rules! include_meshes_bytes_align_as {
    ($align_ty:ty, $path:literal) => {{
        static ALIGNED: &AlignedTo<$align_ty, [u8]> = &AlignedTo {
            _align: [],
            bytes: *include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/raw_meshes/",
                $path
            )),
        };
        &ALIGNED.bytes
    }};
}

pub fn cube() -> MeshData {
    MeshData::new(
        &[
            // +X face
            [0.5, -0.5, -0.5],
            [0.5, 0.5, -0.5],
            [0.5, 0.5, 0.5],
            [0.5, -0.5, 0.5],
            // -X face
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [-0.5, 0.5, -0.5],
            [-0.5, -0.5, -0.5],
            // +Y face
            [-0.5, 0.5, -0.5],
            [-0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, 0.5, -0.5],
            // -Y face
            [-0.5, -0.5, 0.5],
            [-0.5, -0.5, -0.5],
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            // +Z face
            [-0.5, -0.5, 0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5],
            [-0.5, 0.5, 0.5],
            // -Z face
            [0.5, -0.5, -0.5],
            [-0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [0.5, 0.5, -0.5],
        ],
        &[
            // +X
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            // -X
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            // +Y
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            // -Y
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            // +Z
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            // -Z
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
        ],
        &[
            // +X
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
            // -X
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
            // +Y
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
            // -Y
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
            // +Z
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
            // -Z
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
        ],
        &[
            0, 1, 2, 2, 3, 0, // +X
            4, 5, 6, 6, 7, 4, // -X
            8, 9, 10, 10, 11, 8, // +Y
            12, 13, 14, 14, 15, 12, // -Y
            16, 17, 18, 18, 19, 16, // +Z
            20, 21, 22, 22, 23, 20, // -Z
        ],
    )
}

pub fn cone() -> MeshData {
    MeshData::new(
        cast_slice(include_meshes_bytes_align_as!(f32, "cone/positions.bin")),
        cast_slice(include_meshes_bytes_align_as!(f32, "cone/normals.bin")),
        cast_slice(include_meshes_bytes_align_as!(f32, "cone/texcoords.bin")),
        cast_slice(include_meshes_bytes_align_as!(u32, "cone/indices.bin")),
    )
}

pub fn cylinder() -> MeshData {
    MeshData::new(
        cast_slice(include_meshes_bytes_align_as!(
            f32,
            "cylinder/positions.bin"
        )),
        cast_slice(include_meshes_bytes_align_as!(f32, "cylinder/normals.bin")),
        cast_slice(include_meshes_bytes_align_as!(
            f32,
            "cylinder/texcoords.bin"
        )),
        cast_slice(include_meshes_bytes_align_as!(u32, "cylinder/indices.bin")),
    )
}

pub fn klein() -> MeshData {
    MeshData::new(
        cast_slice(include_meshes_bytes_align_as!(f32, "klein/positions.bin")),
        cast_slice(include_meshes_bytes_align_as!(f32, "klein/normals.bin")),
        cast_slice(include_meshes_bytes_align_as!(f32, "klein/texcoords.bin")),
        cast_slice(include_meshes_bytes_align_as!(u32, "klein/indices.bin")),
    )
}

pub fn pen() -> MeshData {
    MeshData::new(
        cast_slice(include_meshes_bytes_align_as!(f32, "pen/positions.bin")),
        cast_slice(include_meshes_bytes_align_as!(f32, "pen/normals.bin")),
        cast_slice(include_meshes_bytes_align_as!(f32, "pen/texcoords.bin")),
        cast_slice(include_meshes_bytes_align_as!(u32, "pen/indices.bin")),
    )
}

pub fn rock() -> MeshData {
    MeshData::new(
        cast_slice(include_meshes_bytes_align_as!(f32, "rock/positions.bin")),
        cast_slice(include_meshes_bytes_align_as!(f32, "rock/normals.bin")),
        cast_slice(include_meshes_bytes_align_as!(f32, "rock/texcoords.bin")),
        cast_slice(include_meshes_bytes_align_as!(u32, "rock/indices.bin")),
    )
}

// pub fn sphere() -> MeshData {
//     let positions = cast_slice(include_meshes_bytes_align_as!(f32, "sphere/positions.bin"));
//
//     MeshData {
//         positions,
//         normals: cast_slice(include_meshes_bytes_align_as!(f32, "sphere/normals.bin")),
//         texcoords: cast_slice(include_meshes_bytes_align_as!(f32, "sphere/texcoords.bin")),
//         indices: cast_slice(include_meshes_bytes_align_as!(u32, "sphere/indices.bin")),
//         aabb: AABB::new(positions),
//     }
// }

pub fn sphere() -> MeshData {
    MeshData::new(
        cast_slice(include_meshes_bytes_align_as!(f32, "sphere/positions.bin")),
        cast_slice(include_meshes_bytes_align_as!(f32, "sphere/normals.bin")),
        cast_slice(include_meshes_bytes_align_as!(f32, "sphere/texcoords.bin")),
        cast_slice(include_meshes_bytes_align_as!(u32, "sphere/indices.bin")),
    )
}

pub fn tetrahedron() -> MeshData {
    MeshData::new(
        cast_slice(include_meshes_bytes_align_as!(
            f32,
            "tetrahedron/positions.bin"
        )),
        cast_slice(include_meshes_bytes_align_as!(
            f32,
            "tetrahedron/normals.bin"
        )),
        cast_slice(include_meshes_bytes_align_as!(
            f32,
            "tetrahedron/texcoords.bin"
        )),
        cast_slice(include_meshes_bytes_align_as!(
            u32,
            "tetrahedron/indices.bin"
        )),
    )
}

pub fn torus() -> MeshData {
    MeshData::new(
        cast_slice(include_meshes_bytes_align_as!(f32, "torus/positions.bin")),
        cast_slice(include_meshes_bytes_align_as!(f32, "torus/normals.bin")),
        cast_slice(include_meshes_bytes_align_as!(f32, "torus/texcoords.bin")),
        cast_slice(include_meshes_bytes_align_as!(u32, "torus/indices.bin")),
    )
}

pub fn billboard() -> MeshData {
    MeshData::new(
        &[
            [-0.5, -0.5, 0.0],
            [0.5, -0.5, 0.0],
            [0.5, 0.5, 0.0],
            [-0.5, 0.5, 0.0],
        ],
        &[
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ],
        &[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
        &[0, 1, 2, 2, 3, 0],
    )
}

pub fn plane(n_points: usize) -> MeshData {
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(n_points * n_points);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(n_points * n_points);
    for i in 0..n_points {
        for j in 0..n_points {
            positions.push([
                i as f32 / (n_points - 1) as f32 * 300.0,
                0.0,
                j as f32 / (n_points - 1) as f32 * 300.0,
            ]);
            normals.push([0.0, 1.0, 0.0]);
        }
    }

    let positions: &'static [[f32; 3]] = positions.leak();
    let normals: &'static [[f32; 3]] = normals.leak();

    let mut indices: Vec<u32> = Vec::with_capacity((n_points - 1) * (n_points - 1) * 6);
    for i in 0..n_points - 1 {
        for j in 0..n_points - 1 {
            let v0 = i * n_points + j;
            let v1 = v0 + 1;
            let v2 = (i + 1) * n_points + j;
            let v3 = v2 + 1;

            indices.push(v0 as u32);
            indices.push(v2 as u32);
            indices.push(v1 as u32);

            indices.push(v1 as u32);
            indices.push(v2 as u32);
            indices.push(v3 as u32);
        }
    }

    let indices: &'static [u32] = indices.leak();

    let mut texcoords: Vec<[f32; 2]> = Vec::with_capacity(n_points * n_points);
    for i in 0..n_points {
        for j in 0..n_points {
            texcoords.push([
                i as f32 / (n_points - 1) as f32,
                j as f32 / (n_points - 1) as f32,
            ]);
        }
    }

    let texcoords: &'static [[f32; 2]] = texcoords.leak();

    MeshData::new(positions, normals, texcoords, indices)
}

pub fn boat() -> MeshData {
    load_mesh("assets/raw_meshes/Boat.glb", None).expect("Error al cargar Mesh")
}

pub fn empty_island() -> MeshData {
    load_mesh("assets/raw_meshes/EmptyIsland.glb", None).expect("Error al cargar Mesh")
}

pub fn grass() -> MeshData {
    load_mesh("assets/raw_meshes/Grass.glb", None).expect("Error al cargar Mesh")
}

pub fn island() -> MeshData {
    load_mesh("assets/raw_meshes/Island.glb", None).expect("Error al cargar Mesh")
}

pub fn palm_tree() -> MeshData {
    load_mesh("assets/raw_meshes/PalmTree.glb", None).expect("Error al cargar Mesh")
}

pub fn sail_boat() -> MeshData {
    load_mesh(
        "assets/raw_meshes/SailBoat.glb",
        Some(glam::Quat::from_rotation_x(-FRAC_PI_2)),
    )
    .expect("Error al cargar Mesh")
}

pub fn ship() -> MeshData {
    load_mesh("assets/raw_meshes/Ship.glb", None).expect("Error al cargar Mesh")
}
