use crate::mesh::{AABB, MeshData};
use bytemuck::cast_slice;

#[repr(C)]
struct AlignedTo<Align, Bytes: ?Sized> {
    _align: [Align; 0],
    bytes: Bytes,
}

macro_rules! include_bytes_align_as {
    ($align_ty:ty, $path:literal) => {{
        static ALIGNED: &AlignedTo<$align_ty, [u8]> = &AlignedTo {
            _align: [],
            bytes: *include_bytes!($path),
        };
        &ALIGNED.bytes
    }};
}

pub fn cube() -> MeshData {
    let positions = cast_slice(include_bytes_align_as!(
        f32,
        "../assets/raw_meshes/cube/positions.bin"
    ));

    MeshData {
        positions,
        normals: cast_slice(include_bytes_align_as!(
            f32,
            "../assets/raw_meshes/cube/normals.bin"
        )),
        texcoords: cast_slice(include_bytes_align_as!(
            f32,
            "../assets/raw_meshes/cube/texcoords.bin"
        )),
        indices: cast_slice(include_bytes_align_as!(
            u32,
            "../assets/raw_meshes/cube/indices.bin"
        )),
        aabb: AABB::new(positions),
    }
}

pub fn cone() -> MeshData {
    let positions = cast_slice(include_bytes_align_as!(
        f32,
        "../assets/raw_meshes/cone/positions.bin"
    ));

    MeshData {
        positions,
        normals: cast_slice(include_bytes_align_as!(
            f32,
            "../assets/raw_meshes/cone/normals.bin"
        )),
        texcoords: cast_slice(include_bytes_align_as!(
            f32,
            "../assets/raw_meshes/cone/texcoords.bin"
        )),
        indices: cast_slice(include_bytes_align_as!(
            u32,
            "../assets/raw_meshes/cone/indices.bin"
        )),
        aabb: AABB::new(positions),
    }
}

pub fn cylinder() -> MeshData {
    let positions = cast_slice(include_bytes_align_as!(
        f32,
        "../assets/raw_meshes/cylinder/positions.bin"
    ));

    MeshData {
        positions,
        normals: cast_slice(include_bytes_align_as!(
            f32,
            "../assets/raw_meshes/cylinder/normals.bin"
        )),
        texcoords: cast_slice(include_bytes_align_as!(
            f32,
            "../assets/raw_meshes/cylinder/texcoords.bin"
        )),
        indices: cast_slice(include_bytes_align_as!(
            u32,
            "../assets/raw_meshes/cylinder/indices.bin"
        )),
        aabb: AABB::new(positions),
    }
}

pub fn klein() -> MeshData {
    let positions = cast_slice(include_bytes_align_as!(
        f32,
        "../assets/raw_meshes/klein/positions.bin"
    ));

    MeshData {
        positions,
        normals: cast_slice(include_bytes_align_as!(
            f32,
            "../assets/raw_meshes/klein/normals.bin"
        )),
        texcoords: cast_slice(include_bytes_align_as!(
            f32,
            "../assets/raw_meshes/klein/texcoords.bin"
        )),
        indices: cast_slice(include_bytes_align_as!(
            u32,
            "../assets/raw_meshes/klein/indices.bin"
        )),
        aabb: AABB::new(positions),
    }
}

pub fn pen() -> MeshData {
    let positions = cast_slice(include_bytes_align_as!(
        f32,
        "../assets/raw_meshes/pen/positions.bin"
    ));

    MeshData {
        positions,
        normals: cast_slice(include_bytes_align_as!(
            f32,
            "../assets/raw_meshes/pen/normals.bin"
        )),
        texcoords: cast_slice(include_bytes_align_as!(
            f32,
            "../assets/raw_meshes/pen/texcoords.bin"
        )),
        indices: cast_slice(include_bytes_align_as!(
            u32,
            "../assets/raw_meshes/pen/indices.bin"
        )),
        aabb: AABB::new(positions),
    }
}

pub fn rock() -> MeshData {
    let positions = cast_slice(include_bytes_align_as!(
        f32,
        "../assets/raw_meshes/rock/positions.bin"
    ));

    MeshData {
        positions,
        normals: cast_slice(include_bytes_align_as!(
            f32,
            "../assets/raw_meshes/rock/normals.bin"
        )),
        texcoords: cast_slice(include_bytes_align_as!(
            f32,
            "../assets/raw_meshes/rock/texcoords.bin"
        )),
        indices: cast_slice(include_bytes_align_as!(
            u32,
            "../assets/raw_meshes/rock/indices.bin"
        )),
        aabb: AABB::new(positions),
    }
}

pub fn sphere() -> MeshData {
    let positions = cast_slice(include_bytes_align_as!(
        f32,
        "../assets/raw_meshes/sphere/positions.bin"
    ));

    MeshData {
        positions,
        normals: cast_slice(include_bytes_align_as!(
            f32,
            "../assets/raw_meshes/sphere/normals.bin"
        )),
        texcoords: cast_slice(include_bytes_align_as!(
            f32,
            "../assets/raw_meshes/sphere/texcoords.bin"
        )),
        indices: cast_slice(include_bytes_align_as!(
            u32,
            "../assets/raw_meshes/sphere/indices.bin"
        )),
        aabb: AABB::new(positions),
    }
}

pub fn tetrahedron() -> MeshData {
    let positions = cast_slice(include_bytes_align_as!(
        f32,
        "../assets/raw_meshes/tetrahedron/positions.bin"
    ));

    MeshData {
        positions,
        normals: cast_slice(include_bytes_align_as!(
            f32,
            "../assets/raw_meshes/tetrahedron/normals.bin"
        )),
        texcoords: cast_slice(include_bytes_align_as!(
            f32,
            "../assets/raw_meshes/tetrahedron/texcoords.bin"
        )),
        indices: cast_slice(include_bytes_align_as!(
            u32,
            "../assets/raw_meshes/tetrahedron/indices.bin"
        )),
        aabb: AABB::new(positions),
    }
}

pub fn torus() -> MeshData {
    let positions = cast_slice(include_bytes_align_as!(
        f32,
        "../assets/raw_meshes/torus/positions.bin"
    ));

    MeshData {
        positions,
        normals: cast_slice(include_bytes_align_as!(
            f32,
            "../assets/raw_meshes/torus/normals.bin"
        )),
        texcoords: cast_slice(include_bytes_align_as!(
            f32,
            "../assets/raw_meshes/torus/texcoords.bin"
        )),
        indices: cast_slice(include_bytes_align_as!(
            u32,
            "../assets/raw_meshes/torus/indices.bin"
        )),
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
                i as f32 / (n_points - 1) as f32 * 250.0,
                0.0,
                j as f32 / (n_points - 1) as f32 * 250.0,
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
