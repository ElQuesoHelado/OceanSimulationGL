use glam::{Mat4, Vec3, camera};

use crate::{light::Light, mesh::MeshLibrary, scene::Instance, shader::Shader};

// Figuras 3D
pub struct StandardRenderer {
    shader: Shader,
}
impl StandardRenderer {
    pub fn new(gl: &glow::Context, vertex_path: &str, frag_path: &str) -> Result<Self, String> {
        let shader = Shader::new(gl, vertex_path, frag_path)?;
        Ok(Self { shader })
    }

    pub fn draw(
        &self,
        gl: &glow::Context,
        instances: &[Instance],
        view: &Mat4,
        proj: &Mat4,
        light: Light,
        view_pos: Vec3,
    ) {
        self.shader.activate(gl);
        self.shader.set_mat4(gl, "uView", view);
        self.shader.set_mat4(gl, "uProjection", proj);
        self.shader.set_bool(gl, "uLightingEnabled", light.enabled);
        self.shader.set_vec3(gl, "lightPos", light.pos);
        self.shader.set_vec3(gl, "lightColor", light.color);
        self.shader.set_vec3(gl, "viewPos", view_pos);

        for inst in instances {}

        // for (auto &p : primitives) {
        //   shader->setVec3("uColor", p.color);
        //   shader->setFloat("shininess", p.shininess);
        //   p.draw();
        // }
    }
}
