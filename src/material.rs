use std::fmt;

use crate::texture::TextureLibrary;

#[derive(Clone, Copy, PartialEq)]
pub struct Material {
    pub color: glam::Vec4,
    pub shininess: f32,
    pub texture_id: u32,
}

#[derive(Debug)]
pub enum MaterialError {
    TextureNotFound,
}

impl fmt::Display for MaterialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Textura no existente")
    }
}

impl Material {
    pub fn new(
        texture_library: &TextureLibrary,
        color: glam::Vec4,
        shininess: f32,
        texture_name: &str,
    ) -> Result<Self, MaterialError> {
        let texture_id = texture_library
            .get_id_from_name(texture_name)
            .ok_or(MaterialError::TextureNotFound)?;

        Ok(Self {
            color,
            shininess,
            texture_id,
        })
    }
}
