use super::*;

impl App {
    pub fn about_to_wait_impl(&mut self, _event_loop: &ActiveEventLoop) {
        let Some(state) = self.state.as_mut() else {
            return;
        };

        state.rain.update(&mut state.scene, 0.11f32);

        state.process_input();
        state.input.end_frame();

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
            &state.graph_ctx.gl(),
            &state.floor_gizmo.floor,
            &identity,
            &state.camera.view(),
            &state.camera.projection(),
            state.floor_gizmo.floor_color,
        );
        state.floor_giz_renderer.draw(
            &state.graph_ctx.gl(),
            &state.floor_gizmo.gizmo,
            &identity,
            &state.camera.view(),
            &state.camera.projection(),
            state.floor_gizmo.gizmo_color,
        );

        // if let Some(idx) = state.scene.selected {
        //     let world_pos = state.scene.instances[idx].transform.w_axis.truncate();
        //     state
        //         .gizmo
        //         .draw(&state.renderer.gl, &state.camera, world_pos);
        // }

        state.window.request_redraw();
        state.gl_surface.swap_buffers(&state.gl_context).unwrap();
    }
}
