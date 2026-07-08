use glam::{Mat4, Vec3, Vec4};
use std::fs;
use std::io;
use std::result::Result;

use glow::{HasContext, Program};

pub struct Shader {
    pub program: Program,
}

impl Shader {
    pub fn new(gl: &glow::Context, vertex_path: &str, frag_path: &str) -> Result<Self, String> {
        let vert_src = read_file(vertex_path).map_err(|e| e.to_string())?;
        let frag_src = read_file(frag_path).map_err(|e| e.to_string())?;

        let v_shader = compile_shader(gl, glow::VERTEX_SHADER, &vert_src)?;
        let f_shader = compile_shader(gl, glow::FRAGMENT_SHADER, &frag_src)?;

        let program = create_program(gl, v_shader, f_shader)?;

        Ok(Self { program })
    }

    pub fn drop(&self, gl: &glow::Context) {
        unsafe { gl.delete_program(self.program) };
    }

    pub fn activate(&self, gl: &glow::Context) {
        unsafe { gl.use_program(Some(self.program)) };
    }

    pub fn get_program(&self, gl: &glow::Context) -> &Program {
        &self.program
    }

    pub fn set_bool(&self, gl: &glow::Context, name: &str, value: bool) {
        unsafe {
            let location = gl.get_uniform_location(self.program, name);
            gl.uniform_1_i32(location.as_ref(), value as i32);
        }
    }

    pub fn set_int(&self, gl: &glow::Context, name: &str, value: i32) {
        unsafe {
            let location = gl.get_uniform_location(self.program, name);
            gl.uniform_1_i32(location.as_ref(), value);
        }
    }

    pub fn set_float(&self, gl: &glow::Context, name: &str, value: f32) {
        unsafe {
            let location = gl.get_uniform_location(self.program, name);
            gl.uniform_1_f32(location.as_ref(), value);
        }
    }

    pub fn set_vec3(&self, gl: &glow::Context, name: &str, value: &Vec3) {
        unsafe {
            let location = gl.get_uniform_location(self.program, name);
            gl.uniform_3_f32_slice(location.as_ref(), &value.to_array());
        }
    }

    pub fn set_vec4(&self, gl: &glow::Context, name: &str, value: &Vec4) {
        unsafe {
            let location = gl.get_uniform_location(self.program, name);
            gl.uniform_4_f32_slice(location.as_ref(), &value.to_array());
        }
    }

    pub fn set_mat4(&self, gl: &glow::Context, name: &str, value: &Mat4) {
        unsafe {
            let location = gl.get_uniform_location(self.program, name);
            gl.uniform_matrix_4_f32_slice(location.as_ref(), false, &value.to_cols_array());
        }
    }
}

fn read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

fn compile_shader(
    gl: &glow::Context,
    shader_type: u32,
    source: &str,
) -> Result<glow::Shader, String> {
    unsafe {
        let vs = gl.create_shader(shader_type)?;
        gl.shader_source(vs, source);
        gl.compile_shader(vs);

        if !gl.get_shader_compile_status(vs) {
            return Err(gl.get_shader_info_log(vs));
        }
        Ok(vs)
    }
}

fn create_program(
    gl: &glow::Context,
    v_shader: glow::Shader,
    f_shader: glow::Shader,
) -> Result<glow::Program, String> {
    unsafe {
        let program = gl.create_program()?;
        gl.attach_shader(program, v_shader);
        gl.attach_shader(program, f_shader);

        gl.link_program(program);

        if !gl.get_program_link_status(program) {
            return Err(gl.get_program_info_log(program));
        }

        gl.delete_shader(v_shader);
        gl.delete_shader(f_shader);

        Ok(program)
    }
}
