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

        // Simulations

        //state.simulation.rain.update(&mut state.scene, 0.11f32);

        state
            .simulation
            .ocean
            .update(&mut state.scene.normal_instances, state.time);

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

        state.ocean_renderer.draw(
            &state.graph_ctx,
            &state.scene.ocean_instances,
            &state.camera,
            &state.light,
            state.time,
        );

        let identity = glam::Mat4::IDENTITY;
        // state.floor_giz_renderer.draw(
        //     state.graph_ctx.gl(),
        //     &state.floor_gizmo.floor,
        //     &identity,
        //     &state.camera.view(),
        //     &state.camera.projection(),
        //     state.floor_gizmo.floor_color,
        // );

        state.floor_giz_renderer.draw(
            state.graph_ctx.gl(),
            &state.floor_gizmo.gizmo,
            &identity,
            &state.camera.view(),
            &state.camera.projection(),
            state.floor_gizmo.gizmo_color,
        );

        state
            .skybox_renderer
            .draw(&state.graph_ctx, &state.scene.skybox, &state.camera);

        // state
        //     .skybox_renderer
        //     .draw(&state.graph_ctx, &state.camera, &state.scene.skybox);

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

        let (gl, mesh_library, texture_library) = state.graph_ctx.gl_and_libraries();

        build_ui(
            gl,
            texture_library,
            mesh_library,
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

        state.time += 0.02;
    }
}
