use glam::{Mat4, Vec3, camera};

use crate::{camera::Camera, light::Light, mesh::MeshLibrary, scene::Instance, shader::Shader};

// Figuras 3D
pub struct StandardRenderer {
    shader: Shader,
}
impl StandardRenderer {
    pub fn new(gl: &glow::Context, vertex_path: &str, frag_path: &str) -> Result<Self, String> {
        let shader = Shader::new(gl, vertex_path, frag_path)?;
        Ok(Self { shader })
    }

    pub fn draw(&self, gl: &glow::Context, instances: &[Instance], camera: &Camera, light: &Light) {
        self.shader.activate(gl);
        self.shader.set_mat4(gl, "uView", &camera.view());
        self.shader
            .set_mat4(gl, "uProjection", &camera.projection());
        self.shader.set_bool(gl, "uLightingEnabled", light.enabled);
        self.shader.set_vec3(gl, "uLightPos", light.pos);
        self.shader.set_vec3(gl, "uLightColor", light.color);
        self.shader.set_vec3(gl, "uEye", camera.eye());

        for inst in instances {
            self.shader.set_vec4(gl, "uColor", inst.material.color);
            self.shader
                .set_float(gl, "uShininess", inst.material.shininess);
            //FIXME: Pasar transform de primitive
            //inst.draw();
        }
    }
}
