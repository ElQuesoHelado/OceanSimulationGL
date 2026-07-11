use super::*;

use dear_imgui_rs::{Condition, Ui, WindowFlags};

const MESH_OPTIONS: &[(&str, MeshId)] = &[
    ("Cubo", MeshId::Cube),
    ("Esfera", MeshId::Sphere),
    ("Toro", MeshId::Torus),
    ("Botella", MeshId::Klein),
    ("Roca", MeshId::Rock),
    ("Pen", MeshId::Pen),
    ("Cilindro", MeshId::Cylinder),
    ("Tetraedro", MeshId::Tetrahedron),
    ("Cono", MeshId::Cone),
];

pub fn build_ui(
    gl: &glow::Context,
    texture_library: &TextureLibrary,
    ui_state: &mut UiState,
    ui: &Ui,
    scene: &mut Scene,
    light: &mut Light,
    width: f32,
    height: f32,
) {
    let toolbar_width = 100.0;

    ui.window("Editor3D")
        .position([0.0, 0.0], Condition::Always)
        .size([toolbar_width, height], Condition::Always)
        .flags(WindowFlags::NO_MOVE | WindowFlags::NO_RESIZE | WindowFlags::NO_COLLAPSE)
        .build(|| {
            ui.text("Figuras");
            ui.separator();

            for (label, mesh_id) in MESH_OPTIONS {
                let is_selected = ui_state.mesh_to_draw == *mesh_id;
                if ui.selectable_config(label).selected(is_selected).build() {
                    ui_state.mesh_to_draw = *mesh_id;
                }
            }

            ui.separator();

            let mut color = ui_state.selected_material.color.to_array();
            if ui.color_edit4("Col", &mut color) {
                ui_state.selected_material.color = Vec4::from_array(color);
            }

            if ui.button("Paint")
                && let Some(idx) = ui_state.selected_mesh
            {
                std::mem::swap(
                    &mut scene.normal_instances[idx].material.color,
                    &mut ui_state.buffered_color,
                );
                scene.normal_instances[idx].material.color = ui_state.selected_material.color;
                ui_state.selected_mesh = None;
            }

            ui.separator();
            ui.text("Operaciones");

            // --- Escala ---
            match stepper(ui, "SCL", "l1") {
                Step::Plus => with_selected(ui_state, scene, |p| {
                    p.transform.scale(vec3(1.1, 1.1, 1.1));
                }),
                Step::Minus => with_selected(ui_state, scene, |p| {
                    p.transform.scale(vec3(0.9, 0.9, 0.9));
                }),
                Step::None => {}
            }

            // --- Rotaciones ---
            match stepper(ui, "ROTX", "l2") {
                Step::Plus => with_selected(ui_state, scene, |p| {
                    p.transform.rotate_x(0.3);
                }),
                Step::Minus => with_selected(ui_state, scene, |p| {
                    p.transform.rotate_x(-0.3);
                }),
                Step::None => {}
            }

            match stepper(ui, "ROTY", "l3") {
                Step::Plus => with_selected(ui_state, scene, |p| {
                    p.transform.rotate_y(0.3);
                }),
                Step::Minus => with_selected(ui_state, scene, |p| {
                    p.transform.rotate_y(-0.3);
                }),
                Step::None => {}
            }

            match stepper(ui, "ROTZ", "l4") {
                Step::Plus => with_selected(ui_state, scene, |p| {
                    p.transform.rotate_z(0.3);
                }),
                Step::Minus => with_selected(ui_state, scene, |p| {
                    p.transform.rotate_z(-0.3);
                }),
                Step::None => {}
            }

            // --- Traslaciones ---
            match stepper(ui, "TRANSX", "l5") {
                Step::Plus => with_selected(ui_state, scene, |p| {
                    p.transform.translate(vec3(1.0, 0.0, 0.0));
                }),
                Step::Minus => with_selected(ui_state, scene, |p| {
                    p.transform.translate(vec3(-1.0, 0.0, 0.0));
                }),
                Step::None => {}
            }

            match stepper(ui, "TRANSY", "l6") {
                Step::Plus => with_selected(ui_state, scene, |p| {
                    p.transform.translate(vec3(0.0, 1.0, 0.0));
                }),
                Step::Minus => with_selected(ui_state, scene, |p| {
                    p.transform.translate(vec3(0.0, -1.0, 0.0));
                }),
                Step::None => {}
            }

            match stepper(ui, "TRANSZ", "l7") {
                Step::Plus => with_selected(ui_state, scene, |p| {
                    p.transform.translate(vec3(0.0, 0.0, 1.0));
                }),
                Step::Minus => with_selected(ui_state, scene, |p| {
                    p.transform.translate(vec3(0.0, 0.0, -1.0));
                }),
                Step::None => {}
            }

            // --- Miscs ---
            ui.separator();
            ui.text("MISCS");

            if ui.checkbox("Wireframe", &mut ui_state.wireframe_enabled) {
                unsafe {
                    gl.polygon_mode(
                        glow::FRONT_AND_BACK,
                        if ui_state.wireframe_enabled {
                            glow::LINE
                        } else {
                            glow::FILL
                        },
                    );
                }
            }
            ui.checkbox("Lighting", &mut light.enabled);

            if ui.button("DUPE")
                && let Some(idx) = ui_state.selected_mesh
            {
                std::mem::swap(
                    &mut scene.normal_instances[idx].material.color,
                    &mut ui_state.buffered_color,
                );
                let dup = scene.normal_instances[idx].clone();
                scene.normal_instances.push(dup);
                ui_state.selected_mesh = None;
            }
            if ui.button("DEL")
                && let Some(idx) = ui_state.selected_mesh
            {
                std::mem::swap(
                    &mut scene.normal_instances[idx].material.color,
                    &mut ui_state.buffered_color,
                );
                scene.normal_instances.remove(idx);
                ui_state.selected_mesh = None;
            }

            ui.separator();
            ui.text("Textures");

            let list_height = 8.0 * ui.text_line_height_with_spacing();
            if let Some(_token) = ui
                .list_box_config("##Textures")
                .size([-f32::MIN_POSITIVE, list_height])
                .begin(ui)
            {
                let texture_library = &texture_library;
                for texture_name in texture_library.names.keys() {
                    let Some(tex_id) = texture_library.get_id_from_name(texture_name) else {
                        continue;
                    };

                    if ui.selectable(texture_name) {
                        ui_state.selected_material.texture_id = tex_id;
                        if let Some(idx) = ui_state.selected_mesh {
                            scene.normal_instances[idx].material.texture_id = tex_id;
                        }
                    }
                }
            }
        });
}

fn with_selected(ui_state: &UiState, scene: &mut Scene, f: impl FnOnce(&mut Instance)) {
    if let Some(idx) = ui_state.selected_mesh {
        f(&mut scene.normal_instances[idx]);
    }
}

enum Step {
    Plus,
    Minus,
    None,
}

fn stepper(ui: &Ui, label: &str, id: &str) -> Step {
    ui.text(label);
    ui.same_line();

    let mut result = Step::None;

    if ui.button(&format!("+##{id}_plus")) {
        result = Step::Plus;
    }
    ui.same_line();
    if ui.button(&format!("-##{id}_minus")) {
        result = Step::Minus;
    }

    result
}
