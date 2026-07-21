use glow::HasContext;
use image::{ImageBuffer, Rgba};
use std::io;
use std::{collections::HashMap, fs};

pub struct TextureLibrary {
    pub names: HashMap<String, u32>,
    pub textures: HashMap<u32, Texture>, // u32 como Ids
    pub cube_maps: HashMap<u32, CubeMap>,
}

impl TextureLibrary {
    pub fn get_id_from_name(&self, name: &str) -> Option<u32> {
        self.names.get(name).copied()
    }

    pub fn get_texture_from_id(&self, id: u32) -> Option<&Texture> {
        self.textures.get(&id)
    }

    pub fn get_cubemap_from_id(&self, id: u32) -> Option<&CubeMap> {
        self.cube_maps.get(&id)
    }

    pub fn get_texture_from_name(&self, name: &str) -> Option<&Texture> {
        self.get_texture_from_id(self.get_id_from_name(name)?)
    }

    pub fn new(gl: &glow::Context, dir_path: &str) -> Self {
        let mut names: HashMap<String, u32> = HashMap::new();
        let mut textures: HashMap<u32, Texture> = HashMap::new();
        let mut cube_maps: HashMap<u32, CubeMap> = HashMap::new();

        let entries = match fs::read_dir(dir_path) {
            Ok(entries) => entries,
            Err(_) => {
                return Self {
                    names,
                    textures,
                    cube_maps,
                };
            }
        };

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            let file_type = match entry.file_type() {
                Ok(ft) => ft,
                Err(_) => continue,
            };

            let path = entry.path().to_string_lossy().to_string();
            let name = entry
                .path()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .into_owned();

            if file_type.is_dir() {
                match CubeMap::new(gl, &path) {
                    Ok(cube_map) => {
                        //println!("Cubemap name: {}", name);
                        let len_map = cube_maps.len() as u32;
                        cube_maps.insert(len_map, cube_map);
                        names.insert(name, len_map);
                    }
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                }
            } else if file_type.is_file() {
                match Texture::new(gl, &path) {
                    Ok(texture) => {
                        let len_map = textures.len() as u32;
                        textures.insert(len_map, texture);
                        names.insert(name, len_map);
                    }
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                }
            }
        }
        Self {
            names,
            textures,
            cube_maps,
        }
    }
}

pub struct Texture {
    pub id: glow::Texture,
    pub width: i32,
    pub height: i32,
    pub channels: i32,
    pub path: String,
}

impl Texture {
    pub fn new(gl: &glow::Context, path: &str) -> Result<Self, String> {
        // println!("{}", path);
        let img = image::open(path).map_err(|e| e.to_string())?;
        let rgba = img.flipv().into_rgba8();
        //let rgba = img.into_rgba8();

        let (width, height) = rgba.dimensions();

        unsafe {
            let texture = gl.create_texture()?;
            gl.bind_texture(glow::TEXTURE_2D, Some(texture));
            gl.bind_texture(glow::TEXTURE_2D, None);
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

pub struct CubeMap {
    pub id: glow::Texture,
}

impl CubeMap {
    pub fn new(gl: &glow::Context, dir_path: &str) -> Result<Self, String> {
        let posx = format!("{}/posx.png", dir_path);
        let negx = format!("{}/negx.png", dir_path);
        let posy = format!("{}/posy.png", dir_path);
        let negy = format!("{}/negy.png", dir_path);
        let posz = format!("{}/posz.png", dir_path);
        let negz = format!("{}/negz.png", dir_path);

        let faces: [(u32, &str); 6] = [
            (glow::TEXTURE_CUBE_MAP_POSITIVE_X, &posx),
            (glow::TEXTURE_CUBE_MAP_NEGATIVE_X, &negx),
            (glow::TEXTURE_CUBE_MAP_POSITIVE_Y, &posy),
            (glow::TEXTURE_CUBE_MAP_NEGATIVE_Y, &negy),
            (glow::TEXTURE_CUBE_MAP_POSITIVE_Z, &posz),
            (glow::TEXTURE_CUBE_MAP_NEGATIVE_Z, &negz),
        ];

        unsafe {
            let texture = gl.create_texture()?;

            gl.bind_texture(glow::TEXTURE_CUBE_MAP, Some(texture));
            // gl.bind_texture(glow::TEXTURE_CUBE_MAP, None);

            for (cube_map_orientation, path) in &faces {
                let img = image::open(path).map_err(|e| e.to_string())?.into_rgba8();
                let (width, height) = img.dimensions();

                gl.tex_image_2d(
                    *cube_map_orientation,
                    0,
                    glow::RGBA8 as i32,
                    width as i32,
                    height as i32,
                    0,
                    glow::RGBA,
                    glow::UNSIGNED_BYTE,
                    glow::PixelUnpackData::Slice(Some(img.as_raw())),
                );
            }

            gl.tex_parameter_i32(
                glow::TEXTURE_CUBE_MAP,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
            );

            gl.tex_parameter_i32(
                glow::TEXTURE_CUBE_MAP,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );

            gl.tex_parameter_i32(
                glow::TEXTURE_CUBE_MAP,
                glow::TEXTURE_WRAP_S,
                glow::CLAMP_TO_EDGE as i32,
            );

            gl.tex_parameter_i32(
                glow::TEXTURE_CUBE_MAP,
                glow::TEXTURE_WRAP_T,
                glow::CLAMP_TO_EDGE as i32,
            );

            gl.tex_parameter_i32(
                glow::TEXTURE_CUBE_MAP,
                glow::TEXTURE_WRAP_R,
                glow::CLAMP_TO_EDGE as i32,
            );

            Ok(Self { id: texture })
        }
    }

    pub fn drop(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_texture(self.id);
        }
    }
}
