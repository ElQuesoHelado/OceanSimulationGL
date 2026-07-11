use winit::event::{ElementState, Event};

use super::*;

impl App {
    pub fn window_event_impl(
        &mut self,
        event_loop: &ActiveEventLoop,
        id: WindowId,
        event: WindowEvent,
    ) {
        let Some(state) = self.state.as_mut() else {
            return;
        };

        state.graph_ctx.platform.handle_event(
            &mut state.ui_ctx.imgui_ctx,
            &state.window,
            &Event::<()>::WindowEvent {
                window_id: id,
                event: event.clone(),
            },
        );

        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let (dx, dy) = state.input.on_cursor_moved(position.x, position.y);

                let alt = state.input.key_pressed(KeyCode::AltLeft);
                let shift = state.input.key_pressed(KeyCode::ShiftLeft);
                let left_mouse = state.input.mouse_button_pressed(MouseButton::Left);

                if alt && left_mouse {
                    state.camera.orbit(dx as f32, dy as f32);
                } else if shift && left_mouse {
                    state.camera.pan(dx as f32, dy as f32);
                }
            }
            WindowEvent::MouseInput {
                button,
                state: btn_state,
                ..
            } => {
                state.input.on_mouse_button(button, btn_state);

                if button == MouseButton::Left && btn_state == ElementState::Pressed {
                    let alt = state.input.key_pressed(KeyCode::AltLeft);
                    let shift = state.input.key_pressed(KeyCode::ShiftLeft);
                    if !alt && !shift {
                        state.insert_current_primitive();
                    }
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                use winit::keyboard::{KeyCode, PhysicalKey};

                if let PhysicalKey::Code(code) = event.physical_key {
                    state.input.on_keyboard_input(code, event.state);

                    if code == KeyCode::Escape {
                        event_loop.exit();
                    }

                    if event.state == ElementState::Pressed && !event.repeat {
                        // state.handle_selection_key(code);
                    }
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let scroll_amount = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_x, y) => y as f64,
                    winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y,
                };
                state.camera.zoom(scroll_amount as f32);
            }
            WindowEvent::Resized(size) if size.width != 0 && size.height != 0 => {
                state.gl_surface.resize(
                    &state.gl_context,
                    NonZeroU32::new(size.width).unwrap(),
                    NonZeroU32::new(size.height).unwrap(),
                );
                state
                    .graph_ctx
                    .resize(size.width as i32, size.height as i32);
                state
                    .camera
                    .set_aspect(size.width as f32, size.height as f32);
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {}
        }
    }
}
