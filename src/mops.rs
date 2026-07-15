use glam::{Mat4, Quat, Vec3};

#[derive(Clone, Copy, PartialEq)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    pub fn matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }

    pub fn translate(&mut self, t: Vec3) -> &mut Self {
        self.translation += t;
        self
    }

    pub fn scale(&mut self, s: Vec3) -> &mut Self {
        self.scale *= s;
        self
    }

    pub fn rotate_x(&mut self, rads: f32) -> &mut Self {
        self.rotation *= Quat::from_rotation_x(rads);
        self
    }

    pub fn rotate_y(&mut self, rads: f32) -> &mut Self {
        self.rotation *= Quat::from_rotation_y(rads);
        self
    }

    pub fn rotate_z(&mut self, rads: f32) -> &mut Self {
        self.rotation *= Quat::from_rotation_z(rads);
        self
    }

    pub fn rotate_axis(&mut self, rads: f32, axis: Vec3) -> &mut Self {
        self.rotation *= Quat::from_axis_angle(axis, rads);
        self
    }

    pub fn position(&self) -> Vec3 {
        self.translation
    }

    pub fn set_position(&mut self, position: Vec3) -> &Self {
        self.translation = position;
        self
    }

    pub fn set_rotation(&mut self, rotation: Quat) -> &Self {
        self.rotation = rotation;
        self
    }

    pub fn set_pos_x(&mut self, coord: f32) -> &Self {
        self.translation.x = coord;
        self
    }

    pub fn set_pos_y(&mut self, coord: f32) -> &Self {
        self.translation.y = coord;
        self
    }

    pub fn set_pos_z(&mut self, coord: f32) -> &Self {
        self.translation.z = coord;
        self
    }

    pub fn get_x(&self) -> f32 {
        self.translation.x
    }

    pub fn get_y(&self) -> f32 {
        self.translation.y
    }

    pub fn get_z(&self) -> f32 {
        self.translation.z
    }
}
