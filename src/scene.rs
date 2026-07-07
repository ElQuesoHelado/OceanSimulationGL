pub struct MeshId(pub u32);

pub struct Instance {
    pub mesh: MeshId,
    pub pos: glam::Vec3,
    pub color: glam::Vec3,
    pub shininess: f32,
    pub texture: glow::Texture,
}

pub struct Scene {}
