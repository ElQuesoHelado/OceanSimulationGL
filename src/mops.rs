use glam::{Mat4, Vec3};

pub struct Transform {
    pub mat: Mat4,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            mat: Mat4::IDENTITY,
        }
    }

    pub fn rotate_x(&mut self, deg: f32) -> &Self {
        self.mat *= Mat4::from_rotation_x(deg.to_radians());
        self
    }

    pub fn rotate_y(&mut self, deg: f32) -> &Self {
        self.mat *= Mat4::from_rotation_y(deg.to_radians());
        self
    }

    pub fn rotate_z(&mut self, deg: f32) -> &Self {
        self.mat *= Mat4::from_rotation_z(deg.to_radians());
        self
    }

    pub fn rotate_axis(&mut self, deg: f32, axis: Vec3) -> &Self {
        self.mat *= Mat4::from_axis_angle(axis, deg.to_radians());
        self
    }

    pub fn trans(&mut self, t: Vec3) -> &Self {
        self.mat *= Mat4::from_translation(t);
        self
    }

    pub fn position(&self) -> Vec3 {
        self.mat.w_axis.truncate()
    }

    pub fn set_position(&mut self, position: Vec3) -> &Self {
        self.mat.w_axis.x = position.x;
        self.mat.w_axis.y = position.y;
        self.mat.w_axis.z = position.z;
        self
    }
}
