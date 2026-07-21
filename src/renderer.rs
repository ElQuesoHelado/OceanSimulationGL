use glam::{Mat4, Vec3, camera, vec4};
use glow::HasContext;

use crate::{
    app::GraphicsContext,
    camera::Camera,
    light::Light,
    material::Material,
    meshes::mesh::{MeshHandle, MeshLibrary, SimpleMesh},
    scene::{self, Instance, Scene, Skybox},
    shader::Shader,
    simulations::{Simulation, ocean::Wave},
    texture::TextureLibrary,
};

// Figuras 3D
pub struct StandardRenderer {
    shader: Shader,
}
impl StandardRenderer {
    pub fn new(gl: &glow::Context, vertex_path: &str, frag_path: &str) -> Result<Self, String> {
        let shader = Shader::new(gl, vertex_path, frag_path)?;
        shader.activate(gl);
        shader.set_int(gl, "uTexture", 0);

        Ok(Self { shader })
    }

    pub fn draw(
        &self,
        ctx: &GraphicsContext,
        instances: &[Instance],
        camera: &Camera,
        light: &Light,
    ) {
        let gl = ctx.gl();

        self.shader.activate(gl);
        self.shader.set_mat4(gl, "uView", &camera.view());
        self.shader
            .set_mat4(gl, "uProjection", &camera.projection());
        self.shader.set_bool(gl, "uLightingEnabled", light.enabled);
        self.shader.set_vec3(gl, "uLightPos", &light.pos);
        self.shader.set_vec3(gl, "uLightColor", &light.color);
        self.shader.set_vec3(gl, "uEye", &camera.eye);

        for inst in instances {
            self.shader.set_mat4(gl, "uModel", &inst.transform.matrix());
            self.shader.set_vec4(gl, "uColor", &inst.material.color);
            self.shader
                .set_float(gl, "uShininess", inst.material.shininess);

            unsafe {
                gl.active_texture(glow::TEXTURE0);
                gl.bind_texture(
                    glow::TEXTURE_2D,
                    ctx.texture_library
                        .get_texture_from_id(inst.material.texture_id)
                        .map(|tex| tex.id),
                );
            }

            match ctx.mesh_library.get(inst.mesh_handle) {
                Some(v) => v.draw(gl),
                None => continue,
            };
        }
    }
}

pub struct BillboardRenderer {
    shader: Shader,
    floor_vertices: [Vec3; 6],
    giz_pts: [Vec3; 6],
}

impl BillboardRenderer {
    pub fn new(gl: &glow::Context, vertex_path: &str, frag_path: &str) -> Result<Self, String> {
        let shader = Shader::new(gl, vertex_path, frag_path)?;

        shader.activate(gl);
        shader.set_int(gl, "uTexture", 0);

        Ok(Self {
            shader,
            floor_vertices: [
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(1000.0, 0.0, 0.0),
                Vec3::new(1000.0, 0.0, 1000.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(1000.0, 0.0, 1000.0),
                Vec3::new(0.0, 0.0, 1000.0),
            ],
            giz_pts: [
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(1000.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1000.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1000.0),
            ],
        })
    }

    pub fn draw(&self, ctx: &GraphicsContext, instances: &[Instance], camera: &Camera) {
        let gl = ctx.gl();

        self.shader.activate(gl);

        self.shader.set_mat4(gl, "uView", &camera.view());

        self.shader
            .set_mat4(gl, "uProjection", &camera.projection());

        unsafe {
            gl.enable(glow::BLEND);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);

            // Seguimos usando el depth buffer
            gl.depth_mask(true);
        }

        for inst in instances {
            self.shader.set_mat4(gl, "uModel", &inst.transform.matrix());

            self.shader.set_vec4(gl, "uColor", &inst.material.color);

            unsafe {
                gl.active_texture(glow::TEXTURE0);

                gl.bind_texture(
                    glow::TEXTURE_2D,
                    ctx.texture_library
                        .get_texture_from_id(inst.material.texture_id)
                        .map(|tex| tex.id),
                );
            }

            if let Some(mesh) = ctx.mesh_library.get(inst.mesh_handle) {
                mesh.draw(gl);
            }
        }

        unsafe {
            gl.disable(glow::BLEND);
        }
    }
}

pub struct SimpleColorRenderer {
    shader: Shader,
}

impl SimpleColorRenderer {
    pub fn new(gl: &glow::Context, vertex_path: &str, frag_path: &str) -> Result<Self, String> {
        Ok(Self {
            shader: Shader::new(gl, vertex_path, frag_path)?,
        })
    }

    pub fn draw(
        &self,
        gl: &glow::Context,
        mesh: &SimpleMesh,
        model: &glam::Mat4,
        view: &glam::Mat4,
        proj: &glam::Mat4,
        color: glam::Vec3,
    ) {
        self.shader.activate(gl);
        self.shader.set_mat4(gl, "uModel", model);
        self.shader.set_mat4(gl, "uView", view);
        self.shader.set_mat4(gl, "uProjection", proj);
        self.shader.set_vec3(gl, "uColor", &color);
        mesh.draw(gl);
    }
}

pub struct OceanRenderer {
    shader: Shader,
    instance_id: usize,
}

impl OceanRenderer {
    pub fn new(
        gl: &glow::Context,
        vertex_path: &str,
        frag_path: &str,
        waves: &[Wave],
        scene: &mut Scene,
        texture_library: &TextureLibrary,
        mesh_library: &MeshLibrary,
    ) -> Result<Self, String> {
        let shader = Shader::new(gl, vertex_path, frag_path)?;
        shader.activate(gl);
        shader.set_int(gl, "uTexture", 0);

        for (i, wave) in waves.iter().enumerate() {
            shader.set_float(gl, &format!("waves[{}].amplitude", i), wave.amplitude);

            shader.set_float(gl, &format!("waves[{}].frequency", i), wave.frequency);

            shader.set_float(gl, &format!("waves[{}].direction", i), wave.direction);

            shader.set_float(gl, &format!("waves[{}].phase", i), wave.phase);
        }

        shader.set_int(gl, "waveCount", waves.len() as i32);

        let material = Material::new(texture_library, vec4(1., 1., 1., 1.), 128., "ocean")
            .map_err(|e| e.to_string())?;

        let ocean_instance = mesh_library
            .instantiate_from_name("plane", &material, None, None)
            .ok_or("No se pudo crear ocean")?;

        let instance_id = scene.add_ocean_instance(ocean_instance);

        Ok(Self {
            shader,
            instance_id,
        })
    }

    pub fn draw(
        &mut self,
        ctx: &GraphicsContext,
        instances: &[Instance],
        camera: &Camera,
        light: &Light,
        time: f32,
    ) {
        let gl = ctx.gl();

        self.shader.activate(gl);
        self.shader.set_mat4(gl, "uView", &camera.view());
        self.shader
            .set_mat4(gl, "uProjection", &camera.projection());
        self.shader.set_bool(gl, "uLightingEnabled", light.enabled);
        self.shader.set_vec3(gl, "uLightPos", &light.pos);
        self.shader.set_vec3(gl, "uLightColor", &light.color);
        self.shader.set_vec3(gl, "uEye", &camera.eye);
        self.shader.set_float(gl, "time", time);

        for inst in instances {
            // let inst = &instances[self.instance_id];

            self.shader.set_mat4(gl, "uModel", &inst.transform.matrix());
            self.shader.set_vec4(gl, "uColor", &inst.material.color);
            self.shader
                .set_float(gl, "uShininess", inst.material.shininess);

            unsafe {
                gl.active_texture(glow::TEXTURE0);
                gl.bind_texture(
                    glow::TEXTURE_2D,
                    ctx.texture_library
                        .get_texture_from_id(inst.material.texture_id)
                        .map(|tex| tex.id),
                );
            }

            if let Some(v) = ctx.mesh_library.get(inst.mesh_handle) {
                v.draw(gl);
            };
        }
    }
}

pub struct SkyboxRenderer {
    shader: Shader,
}
impl SkyboxRenderer {
    pub fn new(gl: &glow::Context, vertex_path: &str, frag_path: &str) -> Result<Self, String> {
        let shader = Shader::new(gl, vertex_path, frag_path)?;
        shader.activate(gl);
        shader.set_int(gl, "uSkybox", 0);

        Ok(Self { shader })
    }

    pub fn draw(&self, ctx: &GraphicsContext, skybox_instance: &Option<Skybox>, camera: &Camera) {
        let Some(skybox_instance) = skybox_instance else {
            return;
        };

        let gl = ctx.gl();
        self.shader.activate(gl);

        unsafe {
            gl.depth_func(glow::LEQUAL);
        }

        self.shader.set_mat4(gl, "uView", &camera.view());
        self.shader
            .set_mat4(gl, "uProjection", &camera.projection());

        unsafe {
            gl.active_texture(glow::TEXTURE0);
            gl.bind_texture(
                glow::TEXTURE_CUBE_MAP,
                ctx.texture_library
                    .get_cubemap_from_id(skybox_instance.cube_map_id)
                    .map(|tex| tex.id),
            );
        }

        if let Some(v) = ctx.mesh_library.get(skybox_instance.mesh_id) {
            v.draw(gl)
        };

        unsafe {
            gl.depth_func(glow::LESS);
        }
    }
}
