use glam::Mat4;

use super::*;

fn un_project(win: Vec3, view: Mat4, proj: Mat4, viewport: (f32, f32, f32, f32)) -> Vec3 {
    let inverse = (proj * view).inverse();

    let mut ndc = Vec4::new(win.x, win.y, win.z, 1.0);
    ndc.x = (ndc.x - viewport.0) / viewport.2;
    ndc.y = (ndc.y - viewport.1) / viewport.3;
    ndc = ndc * 2.0 - Vec4::ONE;

    let world = inverse * ndc;
    world.truncate() / world.w
}

pub fn closest_hit_planes(
    mouse_x: f32,
    mouse_y: f32,
    width: f32,
    height: f32,
    camera_position: Vec3,
    view: Mat4,
    proj: Mat4,
) -> Vec3 {
    let viewport = (0.0, 0.0, width, height);

    let near_point = un_project(
        Vec3::new(mouse_x, height - mouse_y, 0.0),
        view,
        proj,
        viewport,
    );
    let far_point = un_project(
        Vec3::new(mouse_x, height - mouse_y, 1000.0),
        view,
        proj,
        viewport,
    );

    let origin = camera_position;
    let direction = (far_point - near_point).normalize();

    let t_xz = -origin.y / direction.y;
    let t_yz = -origin.x / direction.x;
    let t_xy = -origin.z / direction.z;

    let hit_xz = origin + t_xz * direction;
    let hit_yz = origin + t_yz * direction;
    let hit_xy = origin + t_xy * direction;

    let dist_xy = hit_xy.distance(origin);
    let dist_xz = hit_xz.distance(origin);
    let dist_yz = hit_yz.distance(origin);

    if dist_xy <= dist_xz && dist_xy <= dist_yz {
        hit_xy
    } else if dist_xz <= dist_yz {
        hit_xz
    } else {
        hit_yz
    }
}

impl AppState {
    pub fn insert_current_primitive(&mut self, mouse_x: f32, mouse_y: f32) {
        let mut instance =
            Instance::new(self.ui_state.mesh_to_draw, self.ui_state.selected_material);

        instance.transform.translate(closest_hit_planes(
            mouse_x,
            mouse_y,
            self.window.inner_size().width as f32,
            self.window.inner_size().height as f32,
            self.camera.eye(),
            self.camera.view(),
            self.camera.projection(),
        ));

        self.scene.normal_instances.push(instance);
    }
}
