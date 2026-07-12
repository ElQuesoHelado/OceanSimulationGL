use glam::{Mat4, Vec3};

#[derive(Clone, Copy, PartialEq)]
pub struct Transform {
    pub mat: Mat4,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            mat: Mat4::IDENTITY,
        }
    }

    pub fn translate(&mut self, t: Vec3) -> &mut Self {
        self.mat *= Mat4::from_translation(t);
        self
    }

    pub fn scale(&mut self, s: Vec3) -> &mut Self {
        self.mat *= Mat4::from_scale(s);
        self
    }

    pub fn rotate_x(&mut self, rads: f32) -> &mut Self {
        self.mat *= Mat4::from_rotation_x(rads);
        self
    }

    pub fn rotate_y(&mut self, rads: f32) -> &mut Self {
        self.mat *= Mat4::from_rotation_y(rads);
        self
    }

    pub fn rotate_z(&mut self, rads: f32) -> &mut Self {
        self.mat *= Mat4::from_rotation_z(rads);
        self
    }

    pub fn rotate_axis(&mut self, rads: f32, axis: Vec3) -> &mut Self {
        self.mat *= Mat4::from_axis_angle(axis, rads);
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
