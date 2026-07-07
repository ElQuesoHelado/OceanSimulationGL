use glow::HasContext;
use std::io;
use std::{collections::HashMap, fs};

pub struct TextureLibrary {
    names: HashMap<String, u32>,
    textures: HashMap<u32, Texture>, // u32 como Ids
}

impl TextureLibrary {
    pub fn get_id_from_name(&self, name: &str) -> Option<u32> {
        self.names.get(name).copied()
    }

    pub fn get_texture_from_id(&self, id: u32) -> Option<&Texture> {
        self.textures.get(&id)
    }

    pub fn get_texture_from_name(&self, name: &str) -> Option<&Texture> {
        self.get_texture_from_id(self.get_id_from_name(name)?)
    }

    pub fn new(gl: &glow::Context, path: &str) -> Self {
        let mut names: HashMap<String, u32> = HashMap::new();
        let mut textures: HashMap<u32, Texture> = HashMap::new();

        let entries = match fs::read_dir(path) {
            Ok(entries) => entries,
            Err(_) => return Self { names, textures },
        };

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            if !entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                continue;
            }

            let name = entry
                .path()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .into_owned();

            match Texture::new(gl, path) {
                Ok(texture) => {
                    let len_map = textures.len() as u32;
                    textures.insert(len_map, texture);
                    names.insert(name, len_map);
                }
                Err(_) => {
                    continue;
                }
            }
        }
        Self { names, textures }
    }
}

pub struct Texture {
    id: glow::Texture,
    width: i32,
    height: i32,
    channels: i32,
    path: String,
}

impl Texture {
    pub fn new(gl: &glow::Context, path: &str) -> Result<Self, String> {
        let img = image::open(path).map_err(|e| e.to_string())?;
        let rgba = img.flipv().into_rgba8();

        let (width, height) = rgba.dimensions();

        unsafe {
            let texture = gl.create_texture()?;
            gl.texture_parameter_i32(texture, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
            gl.texture_parameter_i32(texture, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);

            gl.texture_parameter_i32(texture, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
            gl.texture_parameter_i32(texture, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);

            gl.texture_storage_2d(texture, 1, glow::RGBA8, width as i32, height as i32);

            gl.texture_sub_image_2d(
                texture,
                0,
                0,
                0,
                width as i32,
                height as i32,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(Some(rgba.as_raw())),
            );

            Ok(Self {
                id: texture,
                width: width as i32,
                height: height as i32,
                channels: 4,
                path: path.to_string(),
            })
        }
    }

    pub fn drop(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_texture(self.id);
        }
    }
}
