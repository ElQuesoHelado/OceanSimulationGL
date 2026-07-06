use glow::HasContext;

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
