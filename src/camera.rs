use std::f32::consts::PI;

use glam::camera::rh::view::look_at_mat4;
use glam::{Mat4, Vec3};

//Todo en Rads
pub struct Camera {
    pub target: Vec3,
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub fov_v: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        Self {
            target: Vec3::new(0., 0., 0.),
            distance: 5.,
            yaw: 0.0,
            pitch: 0.3,
            fov_v: 90f32.to_radians(),
            aspect,
            near: 0.05,
            far: 500.0,
        }
    }

    pub fn eye(&self) -> Vec3 {
        Vec3::new(
            self.target.x + self.distance * self.pitch.cos() * self.yaw.cos(),
            self.target.y + self.distance * self.pitch.sin(),
            self.target.z + self.distance * self.pitch.cos() * self.yaw.sin(),
        )
    }

    pub fn view(&self) -> Mat4 {
        look_at_mat4(self.eye(), self.target, Vec3::Y)
    }

    pub fn projection(&self) -> Mat4 {
        glam::camera::rh::proj::opengl::perspective(self.fov_v, self.aspect, self.near, self.far)
    }

    pub fn orbit(&mut self, dx: f32, dy: f32) {
        self.yaw += dx * 0.01;
        self.pitch += dy * 0.01;

        self.pitch = self.pitch.clamp(-PI / 2.1f32, PI / 2.1f32);
    }

    pub fn zoom(&mut self, delta: f32) {
        self.distance *= (1.0 - delta * 0.01);
        self.distance = self.distance.max(0.1);
    }

    pub fn pan(&mut self, dx: f32, dy: f32) {
        let forward = (self.target - self.eye()).normalize();
        let right = (forward.cross(Vec3::new(0., 1., 0.))).normalize();
        let up = right.cross(forward).normalize();

        let speed = self.distance * 0.001;

        self.target -= right * dx * speed;
        self.target += up * dy * speed;
    }

    pub fn set_aspect(&mut self, width: f32, height: f32) {
        if height > 0.0 {
            self.aspect = width / height;
        }
    }
}
