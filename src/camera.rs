use glam::camera::rh::view::look_at_mat4;
use glam::{Mat4, Vec3};

pub struct Camera {
    pub target: Vec3,
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            target: Vec3::new(0., 0., 0.),
            distance: 50.,
            yaw: 45.,
            pitch: 20.,
        }
    }

    pub fn position(&self) -> Vec3 {
        let ry = self.yaw.to_radians();
        let rp = self.pitch.to_radians();

        Vec3::new(
            self.target.x + self.distance * rp.cos() * ry.cos(),
            self.target.y + self.distance * rp.cos(),
            self.target.z + self.distance * rp.cos() * ry.sin(),
        )
    }

    pub fn view(&self) -> Mat4 {
        look_at_mat4(self.position(), self.target, Vec3::new(0., 1., 0.))
    }

    pub fn orbit(&mut self, dx: f32, dy: f32) {
        self.yaw += dx * 0.3;
        self.pitch += dy * 0.3;

        self.pitch = self.pitch.clamp(-89., 89.);
    }

    pub fn zoom(&mut self, delta: f32) {
        self.distance *= (1.0 - delta * 0.1);
        self.distance = self.distance.max(0.1);
    }

    pub fn pan(&mut self, dx: f32, dy: f32) {
        let forward = (self.target - self.position()).normalize();
        let right = (forward.cross(Vec3::new(0., 1., 0.))).normalize();
        let up = right.cross(forward).normalize();

        let speed = self.distance * 0.001;

        self.target -= right * dx * speed;
        self.target += up * dy * speed;
    }
}
