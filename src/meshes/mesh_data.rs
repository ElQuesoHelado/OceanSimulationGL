use std::f32::consts::FRAC_PI_2;

use crate::meshes::{loader::load_mesh, mesh::AABB};
use bytemuck::cast_slice;

// Datos crudos de un mesh(Cube, Sphere, ...)
pub struct MeshData {
    pub positions: &'static [[f32; 3]],
    pub normals: &'static [[f32; 3]],
    pub texcoords: &'static [[f32; 2]],
    pub indices: &'static [u32],
    pub aabb: AABB,
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
    let positions = cast_slice(include_meshes_bytes_align_as!(f32, "cube/positions.bin"));

    MeshData {
        positions,
        normals: cast_slice(include_meshes_bytes_align_as!(f32, "cube/normals.bin")),
        texcoords: cast_slice(include_meshes_bytes_align_as!(f32, "cube/texcoords.bin")),
        indices: cast_slice(include_meshes_bytes_align_as!(u32, "cube/indices.bin")),
        aabb: AABB::new(positions),
    }
}

pub fn cone() -> MeshData {
    let positions = cast_slice(include_meshes_bytes_align_as!(f32, "cone/positions.bin"));

    MeshData {
        positions,
        normals: cast_slice(include_meshes_bytes_align_as!(f32, "cone/normals.bin")),
        texcoords: cast_slice(include_meshes_bytes_align_as!(f32, "cone/texcoords.bin")),
        indices: cast_slice(include_meshes_bytes_align_as!(u32, "cone/indices.bin")),
        aabb: AABB::new(positions),
    }
}

pub fn cylinder() -> MeshData {
    let positions = cast_slice(include_meshes_bytes_align_as!(
        f32,
        "cylinder/positions.bin"
    ));

    MeshData {
        positions,
        normals: cast_slice(include_meshes_bytes_align_as!(f32, "cylinder/normals.bin")),
        texcoords: cast_slice(include_meshes_bytes_align_as!(
            f32,
            "cylinder/texcoords.bin"
        )),
        indices: cast_slice(include_meshes_bytes_align_as!(u32, "cylinder/indices.bin")),
        aabb: AABB::new(positions),
    }
}

pub fn klein() -> MeshData {
    let positions = cast_slice(include_meshes_bytes_align_as!(f32, "klein/positions.bin"));

    MeshData {
        positions,
        normals: cast_slice(include_meshes_bytes_align_as!(f32, "klein/normals.bin")),
        texcoords: cast_slice(include_meshes_bytes_align_as!(f32, "klein/texcoords.bin")),
        indices: cast_slice(include_meshes_bytes_align_as!(u32, "klein/indices.bin")),
        aabb: AABB::new(positions),
    }
}

pub fn pen() -> MeshData {
    let positions = cast_slice(include_meshes_bytes_align_as!(f32, "pen/positions.bin"));

    MeshData {
        positions,
        normals: cast_slice(include_meshes_bytes_align_as!(f32, "pen/normals.bin")),
        texcoords: cast_slice(include_meshes_bytes_align_as!(f32, "pen/texcoords.bin")),
        indices: cast_slice(include_meshes_bytes_align_as!(u32, "pen/indices.bin")),
        aabb: AABB::new(positions),
    }
}

pub fn rock() -> MeshData {
    let positions = cast_slice(include_meshes_bytes_align_as!(f32, "rock/positions.bin"));

    MeshData {
        positions,
        normals: cast_slice(include_meshes_bytes_align_as!(f32, "rock/normals.bin")),
        texcoords: cast_slice(include_meshes_bytes_align_as!(f32, "rock/texcoords.bin")),
        indices: cast_slice(include_meshes_bytes_align_as!(u32, "rock/indices.bin")),
        aabb: AABB::new(positions),
    }
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
    let positions = cast_slice(include_meshes_bytes_align_as!(f32, "sphere/positions.bin"));

    MeshData {
        positions,
        normals: cast_slice(include_meshes_bytes_align_as!(f32, "sphere/normals.bin")),
        texcoords: &[[0f32, 0f32]],
        indices: cast_slice(include_meshes_bytes_align_as!(u32, "sphere/indices.bin")),
        aabb: AABB::new(positions),
    }
}

pub fn tetrahedron() -> MeshData {
    let positions = cast_slice(include_meshes_bytes_align_as!(
        f32,
        "tetrahedron/positions.bin"
    ));

    MeshData {
        positions,
        normals: cast_slice(include_meshes_bytes_align_as!(
            f32,
            "tetrahedron/normals.bin"
        )),
        texcoords: cast_slice(include_meshes_bytes_align_as!(
            f32,
            "tetrahedron/texcoords.bin"
        )),
        indices: cast_slice(include_meshes_bytes_align_as!(
            u32,
            "tetrahedron/indices.bin"
        )),
        aabb: AABB::new(positions),
    }
}

pub fn torus() -> MeshData {
    let positions = cast_slice(include_meshes_bytes_align_as!(f32, "torus/positions.bin"));

    MeshData {
        positions,
        normals: cast_slice(include_meshes_bytes_align_as!(f32, "torus/normals.bin")),
        texcoords: cast_slice(include_meshes_bytes_align_as!(f32, "torus/texcoords.bin")),
        indices: cast_slice(include_meshes_bytes_align_as!(u32, "torus/indices.bin")),
        aabb: AABB::new(positions),
    }
}

pub fn billboard() -> MeshData {
    let positions = &[
        [-0.5, -0.5, 0.0],
        [0.5, -0.5, 0.0],
        [0.5, 0.5, 0.0],
        [-0.5, 0.5, 0.0],
    ];

    MeshData {
        positions,
        normals: &[
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ],
        texcoords: &[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
        indices: &[0, 1, 2, 2, 3, 0],
        aabb: AABB::new(positions),
    }
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

    MeshData {
        positions,
        normals,
        texcoords,
        indices,
        aabb: AABB::new(positions),
    }
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
