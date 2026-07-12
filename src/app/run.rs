use crate::app::ui::build_ui;

use super::*;

impl App {
    pub fn about_to_wait_impl(&mut self, _event_loop: &ActiveEventLoop) {
        let Some(state) = self.state.as_mut() else {
            return;
        };

        if state.ui_state.wireframe_enabled {
            unsafe {
                state
                    .graph_ctx
                    .gl()
                    .polygon_mode(glow::FRONT_AND_BACK, glow::LINE);
            }
        }

        // state.rain.update(&mut state.scene, 0.11f32);

        // state.process_input();
        // state.input.end_frame();

        state.graph_ctx.clear();
        state.standard_renderer.draw(
            &state.graph_ctx,
            &state.scene.normal_instances,
            &state.camera,
            &state.light,
        );

        state.billboard_renderer.draw(
            &state.graph_ctx,
            &state.scene.billboard_instances,
            &state.camera,
        );

        let identity = glam::Mat4::IDENTITY;
        state.floor_giz_renderer.draw(
            state.graph_ctx.gl(),
            &state.floor_gizmo.floor,
            &identity,
            &state.camera.view(),
            &state.camera.projection(),
            state.floor_gizmo.floor_color,
        );

        state.floor_giz_renderer.draw(
            state.graph_ctx.gl(),
            &state.floor_gizmo.gizmo,
            &identity,
            &state.camera.view(),
            &state.camera.projection(),
            state.floor_gizmo.gizmo_color,
        );

        //Render ImGui
        unsafe {
            state
                .graph_ctx
                .gl()
                .polygon_mode(glow::FRONT_AND_BACK, glow::FILL);
        }

        state
            .ui_ctx
            .platform
            .prepare_frame(&state.window, &mut state.ui_ctx.imgui_ctx);

        let ui = state.ui_ctx.imgui_ctx.frame();

        build_ui(
            &state.graph_ctx.texture_library,
            &mut state.ui_state,
            ui,
            &mut state.scene,
            &mut state.light,
            state.window.inner_size().width as f32,
            state.window.inner_size().height as f32,
        );
        state
            .ui_ctx
            .platform
            .prepare_render(&mut state.ui_ctx.imgui_ctx, &state.window);
        let draw_data = state.ui_ctx.imgui_ctx.render();
        state.graph_ctx.renderer.render(draw_data);

        state.window.request_redraw();
        state.gl_surface.swap_buffers(&state.gl_context).unwrap();
    }
}
