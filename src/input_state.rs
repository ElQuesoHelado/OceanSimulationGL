use std::collections::HashSet;
use winit::event::{ElementState, MouseButton};
use winit::keyboard::{Key, KeyCode, NamedKey};

#[derive(Default)]
pub struct InputState {
    pub mouse_x: f64,
    pub mouse_y: f64,
    pub mouse_dx: f64,
    pub mouse_dy: f64,

    keys_down: HashSet<KeyCode>,
    mouse_buttons_down: HashSet<MouseButton>,
}

impl InputState {
    pub fn key_pressed(&self, key: KeyCode) -> bool {
        self.keys_down.contains(&key)
    }

    pub fn mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons_down.contains(&button)
    }

    pub fn end_frame(&mut self) {
        self.mouse_dx = 0.0;
        self.mouse_dy = 0.0;
    }

    pub fn on_keyboard_input(&mut self, key: KeyCode, state: ElementState) {
        match state {
            ElementState::Pressed => {
                self.keys_down.insert(key);
            }
            ElementState::Released => {
                self.keys_down.remove(&key);
            }
        }
    }

    pub fn on_mouse_button(&mut self, button: MouseButton, state: ElementState) {
        match state {
            ElementState::Pressed => {
                self.mouse_buttons_down.insert(button);
            }
            ElementState::Released => {
                self.mouse_buttons_down.remove(&button);
            }
        }
    }

    pub fn on_cursor_moved(&mut self, x: f64, y: f64) {
        self.mouse_dx += x - self.mouse_x;
        self.mouse_dy += y - self.mouse_y;
        self.mouse_x = x;
        self.mouse_y = y;
    }
}
