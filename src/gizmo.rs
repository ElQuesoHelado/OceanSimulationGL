use crate::meshes::mesh::SimpleMesh;

pub struct FloorGizmo {
    pub gizmo: SimpleMesh,
    pub floor: SimpleMesh,
    pub gizmo_color: glam::Vec3,
    pub floor_color: glam::Vec3,
}

impl FloorGizmo {
    pub fn new(gl: &glow::Context) -> Self {
        let giz_pts: [[f32; 3]; 6] = [
            [0.0, 0.0, 0.0],
            [1000.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 1000.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1000.0],
        ];

        let floor_vertices: [[f32; 3]; 6] = [
            [-1000.0, -0.3, -1000.0],
            [1000.0, -0.3, -1000.0],
            [1000.0, -0.3, 1000.0],
            [-1000.0, -0.3, -1000.0],
            [1000.0, -0.3, 1000.0],
            [-1000.0, -0.3, 1000.0],
        ];

        Self {
            gizmo: SimpleMesh::upload(gl, &giz_pts, glow::LINES),
            floor: SimpleMesh::upload(gl, &floor_vertices, glow::TRIANGLES),
            gizmo_color: glam::Vec3::new(0.0, 1.0, 0.0),
            floor_color: glam::Vec3::new(0.5, 0.5, 0.5),
        }
    }

    pub fn draw_gizmo(&self, gl: &glow::Context) {
        self.gizmo.draw(gl);
    }

    pub fn draw_floor(&self, gl: &glow::Context) {
        self.floor.draw(gl);
    }
}
