use glam::Mat4;

use crate::meshes::mesh::MeshKind;

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
    let origin = camera_position;
    let (_, direction) = get_ray(mouse_x, mouse_y, width, height, view, proj);

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

// (near, dir)
fn get_ray(
    mouse_x: f32,
    mouse_y: f32,
    width: f32,
    height: f32,
    view: Mat4,
    proj: Mat4,
) -> (Vec3, Vec3) {
    let viewport = (0.0, 0.0, width, height);

    let near = un_project(
        Vec3::new(mouse_x, height - mouse_y, 0.0),
        view,
        proj,
        viewport,
    );
    let far = un_project(
        Vec3::new(mouse_x, height - mouse_y, 1.0),
        view,
        proj,
        viewport,
    );

    (near, (far - near).normalize())
}

fn ray_hits_aabb(origin: Vec3, dir: Vec3, min: Vec3, max: Vec3) -> bool {
    let mut t_min = f32::MIN;
    let mut t_max = f32::MAX;

    for i in 0..3 {
        let inv = 1.0 / dir[i];
        let mut t1 = (min[i] - origin[i]) * inv;
        let mut t2 = (max[i] - origin[i]) * inv;
        if t1 > t2 {
            std::mem::swap(&mut t1, &mut t2);
        }
        t_min = t_min.max(t1);
        t_max = t_max.min(t2);
    }

    t_max >= t_min && t_max >= 0.0
}

fn ray_to_local(origin: Vec3, dir: Vec3, model: Mat4) -> (Vec3, Vec3) {
    let inv = model.inverse();
    let local_origin = inv.transform_point3(origin);
    let local_dir = inv.transform_vector3(dir); // ya maneja el w=0 por vos
    (local_origin, local_dir.normalize())
}

pub fn select_mesh(
    ray_origin: Vec3,
    ray_dir: Vec3,
    instances: &[Instance],
    mesh_library: &MeshLibrary,
) -> Option<usize> {
    instances
        .iter()
        .enumerate()
        .filter(|(_, m)| {
            let (lo, ld) = ray_to_local(ray_origin, ray_dir, m.transform.matrix());

            let Some(mesh) = mesh_library.get(m.mesh_handle) else {
                return false;
            };

            ray_hits_aabb(lo, ld, mesh.data.aabb.min_point, mesh.data.aabb.max_point)
        })
        .min_by(|(_, a), (_, b)| {
            let mesh_a = mesh_library.get(a.mesh_handle).unwrap();
            let mesh_b = mesh_library.get(b.mesh_handle).unwrap();

            let center_a = mesh_a
                .data
                .aabb
                .min_point
                .midpoint(mesh_a.data.aabb.max_point);
            let center_b = mesh_b
                .data
                .aabb
                .min_point
                .midpoint(mesh_b.data.aabb.max_point);

            let dist_a = a
                .transform
                .matrix()
                .transform_point3(center_a)
                .distance(ray_origin);
            let dist_b = b
                .transform
                .matrix()
                .transform_point3(center_b)
                .distance(ray_origin);

            dist_a.total_cmp(&dist_b)
        })
        .map(|(idx, _)| idx)
}

impl UiState {
    pub fn insert_current_mesh(
        &mut self,
        mouse_x: f32,
        mouse_y: f32,
        scene: &mut Scene,
        mesh_library: &MeshLibrary,
        window: &Window,
        camera: &Camera,
    ) {
        let Some(mut instance) = mesh_library.instantiate_from_handle(
            self.mesh_to_draw,
            &self.selected_material,
            None,
            None,
        ) else {
            return;
        };

        instance.transform.translate(closest_hit_planes(
            mouse_x,
            mouse_y,
            window.inner_size().width as f32,
            window.inner_size().height as f32,
            camera.eye,
            camera.view(),
            camera.projection(),
        ));

        match self.mesh_to_draw.kind {
            MeshKind::Billboard => scene.billboard_instances.push(instance),
            _ => scene.normal_instances.push(instance),
        }
    }

    pub fn select_instance(
        &self,
        mouse_x: f32,
        mouse_y: f32,
        scene: &Scene,
        window: &Window,
        camera: &Camera,
        mesh_library: &MeshLibrary,
    ) -> Option<usize> {
        let (ray_origin, ray_dir) = get_ray(
            mouse_x,
            mouse_y,
            window.inner_size().width as f32,
            window.inner_size().height as f32,
            camera.view(),
            camera.projection(),
        );
        select_mesh(ray_origin, ray_dir, &scene.normal_instances, &mesh_library)
    }

    pub fn clear_selected_instance(&mut self, scene: &mut Scene) {
        if let Some(prev_selected_idx) = self.selected_instance {
            std::mem::swap(
                &mut scene.normal_instances[prev_selected_idx].material.color,
                &mut self.buffered_color,
            );

            self.selected_instance = None;
        };
    }
}
