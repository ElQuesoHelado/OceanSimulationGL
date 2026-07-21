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

        state.ui_ctx.platform.handle_event(
            &mut state.ui_ctx.imgui_ctx,
            &state.window,
            &Event::<()>::WindowEvent {
                window_id: id,
                event: event.clone(),
            },
        );

        let want_mouse = state.ui_ctx.imgui_ctx.io().want_capture_mouse();
        let want_keyboard = state.ui_ctx.imgui_ctx.io().want_capture_keyboard();

        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let (dx, dy) = state.input.on_cursor_moved(position.x, position.y);

                let alt = state.input.key_pressed(KeyCode::AltLeft);
                let shift = state.input.key_pressed(KeyCode::ShiftLeft);
                let left_mouse = state.input.mouse_button_pressed(MouseButton::Left);
                let right_mouse = state.input.mouse_button_pressed(MouseButton::Right);

                if want_mouse {
                    return;
                }

                if (alt && left_mouse) || right_mouse {
                    state.camera.orbit(dx as f32, -dy as f32);
                } else if shift && left_mouse {
                    // state.camera.pan(dx as f32, dy as f32);
                }
            }
            WindowEvent::MouseInput {
                button,
                state: btn_state,
                ..
            } => {
                state.input.on_mouse_button(button, btn_state);

                if want_mouse {
                    return;
                }

                if button == MouseButton::Left && btn_state == ElementState::Pressed {
                    let alt = state.input.key_pressed(KeyCode::AltLeft);
                    let shift = state.input.key_pressed(KeyCode::ShiftLeft);
                    if !alt && !shift {
                        match state.ui_state.click_mode {
                            ClickMode::Insert => state.ui_state.insert_current_mesh(
                                state.input.mouse_x as f32,
                                state.input.mouse_y as f32,
                                &mut state.scene,
                                &state.graph_ctx.mesh_library,
                                &state.window,
                                &state.camera,
                            ),
                            ClickMode::Select => {
                                let Some(inst_idx) = state.ui_state.select_instance(
                                    state.input.mouse_x as f32,
                                    state.input.mouse_y as f32,
                                    &state.scene,
                                    &state.window,
                                    &state.camera,
                                    &state.graph_ctx.mesh_library,
                                ) else {
                                    return;
                                };

                                state.ui_state.clear_selected_instance(&mut state.scene);
                                state.ui_state.selected_instance = Some(inst_idx);

                                std::mem::swap(
                                    &mut state.scene.normal_instances[inst_idx].material.color,
                                    &mut state.ui_state.buffered_color,
                                );
                            }
                        }
                    }
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                use winit::keyboard::{KeyCode, PhysicalKey};

                if want_keyboard {
                    return;
                }

                if let PhysicalKey::Code(code) = event.physical_key {
                    state.input.on_keyboard_input(code, event.state);

                    match code {
                        //KeyCode::Escape => event_loop.exit(),
                        KeyCode::Backquote => {
                            state.ui_state.clear_selected_instance(&mut state.scene)
                        }
                        KeyCode::KeyW => state.camera.fly(1.0, 0.0, 0.0, 0.1),
                        KeyCode::KeyS => state.camera.fly(-1.0, 0.0, 0.0, 0.1),
                        KeyCode::KeyD => state.camera.fly(0.0, 1.0, 0.0, 0.1),
                        KeyCode::KeyA => state.camera.fly(0.0, -1.0, 0.0, 0.1),
                        KeyCode::KeyQ => state.camera.fly(0.0, 0.0, -1.0, 0.1),
                        KeyCode::KeyE => state.camera.fly(0.0, 0.0, 1.0, 0.1),

                        _ => (),
                    }
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                if want_mouse {
                    return;
                }

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
