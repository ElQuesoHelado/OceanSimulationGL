use crate::mesh::MeshData;
use bytemuck::cast_slice;

pub fn cube() -> MeshData {
    MeshData {
        // positions: cast_slice(include_bytes!("../assets/raw_meshes/cube/positions.bin")),
        positions: cast_slice(include_bytes!("../assets/raw_meshes/cube/positions.bin")),
        normals: cast_slice(include_bytes!("../assets/raw_meshes/cube/normals.bin")),
        texcoords: cast_slice(include_bytes!("../assets/raw_meshes/cube/texcoords.bin")),
        indices: cast_slice(include_bytes!("../assets/raw_meshes/cube/indices.bin")),
    }
}

pub fn cone() -> MeshData {
    MeshData {
        positions: cast_slice(include_bytes!("../assets/raw_meshes/cone/positions.bin")),
        normals: cast_slice(include_bytes!("../assets/raw_meshes/cone/normals.bin")),
        texcoords: cast_slice(include_bytes!("../assets/raw_meshes/cone/texcoords.bin")),
        indices: cast_slice(include_bytes!("../assets/raw_meshes/cone/indices.bin")),
    }
}

pub fn cylinder() -> MeshData {
    MeshData {
        positions: cast_slice(include_bytes!(
            "../assets/raw_meshes/cylinder/positions.bin"
        )),
        normals: cast_slice(include_bytes!("../assets/raw_meshes/cylinder/normals.bin")),
        texcoords: cast_slice(include_bytes!(
            "../assets/raw_meshes/cylinder/texcoords.bin"
        )),
        indices: cast_slice(include_bytes!("../assets/raw_meshes/cylinder/indices.bin")),
    }
}

pub fn klein() -> MeshData {
    MeshData {
        positions: cast_slice(include_bytes!("../assets/raw_meshes/klein/positions.bin")),
        normals: cast_slice(include_bytes!("../assets/raw_meshes/klein/normals.bin")),
        texcoords: cast_slice(include_bytes!("../assets/raw_meshes/klein/texcoords.bin")),
        indices: cast_slice(include_bytes!("../assets/raw_meshes/klein/indices.bin")),
    }
}

pub fn pen() -> MeshData {
    MeshData {
        positions: cast_slice(include_bytes!("../assets/raw_meshes/pen/positions.bin")),
        normals: cast_slice(include_bytes!("../assets/raw_meshes/pen/normals.bin")),
        texcoords: cast_slice(include_bytes!("../assets/raw_meshes/pen/texcoords.bin")),
        indices: cast_slice(include_bytes!("../assets/raw_meshes/pen/indices.bin")),
    }
}

pub fn rock() -> MeshData {
    MeshData {
        positions: cast_slice(include_bytes!("../assets/raw_meshes/rock/positions.bin")),
        normals: cast_slice(include_bytes!("../assets/raw_meshes/rock/normals.bin")),
        texcoords: cast_slice(include_bytes!("../assets/raw_meshes/rock/texcoords.bin")),
        indices: cast_slice(include_bytes!("../assets/raw_meshes/rock/indices.bin")),
    }
}

pub fn sphere() -> MeshData {
    MeshData {
        positions: cast_slice(include_bytes!("../assets/raw_meshes/sphere/positions.bin")),
        normals: cast_slice(include_bytes!("../assets/raw_meshes/sphere/normals.bin")),
        texcoords: cast_slice(include_bytes!("../assets/raw_meshes/sphere/texcoords.bin")),
        indices: cast_slice(include_bytes!("../assets/raw_meshes/sphere/indices.bin")),
    }
}

pub fn tetrahedron() -> MeshData {
    MeshData {
        positions: cast_slice(include_bytes!(
            "../assets/raw_meshes/tetrahedron/positions.bin"
        )),
        normals: cast_slice(include_bytes!(
            "../assets/raw_meshes/tetrahedron/normals.bin"
        )),
        texcoords: cast_slice(include_bytes!(
            "../assets/raw_meshes/tetrahedron/texcoords.bin"
        )),
        indices: cast_slice(include_bytes!(
            "../assets/raw_meshes/tetrahedron/indices.bin"
        )),
    }
}

pub fn torus() -> MeshData {
    MeshData {
        positions: cast_slice(include_bytes!("../assets/raw_meshes/torus/positions.bin")),
        normals: cast_slice(include_bytes!("../assets/raw_meshes/torus/normals.bin")),
        texcoords: cast_slice(include_bytes!("../assets/raw_meshes/torus/texcoords.bin")),
        indices: cast_slice(include_bytes!("../assets/raw_meshes/torus/indices.bin")),
    }
}
