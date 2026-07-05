use std::error::Error;
use std::ffi::CString;
use std::num::NonZeroU32;

use raw_window_handle::HasWindowHandle;
use winit::application::ApplicationHandler;
use winit::event::{KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowId};

use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SwapInterval, WindowSurface};

use glow::HasContext;
use glutin_winit::{DisplayBuilder, GlWindow};

pub fn run() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;
    let mut app = App {
        state: None,
        exit_state: Ok(()),
    };
    event_loop.run_app(&mut app)?;
    app.exit_state
}
pub struct App {
    state: Option<AppState>,
    exit_state: Result<(), Box<dyn Error>>,
}

struct AppState {
    window: Window,
    gl_context: PossiblyCurrentContext,
    gl_surface: Surface<WindowSurface>,
    renderer: Renderer,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_some() {
            return;
        }

        let window_attributes =
            Window::default_attributes().with_title("Triangulo con Glow (Escape para salir)");

        let template = ConfigTemplateBuilder::new();
        let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes));

        let (window, gl_config) = match display_builder.build(event_loop, template, |configs| {
            configs
                .reduce(|accum, cfg| {
                    if cfg.num_samples() > accum.num_samples() {
                        cfg
                    } else {
                        accum
                    }
                })
                .unwrap()
        }) {
            Ok((window, gl_config)) => (window.unwrap(), gl_config),
            Err(err) => {
                self.exit_state = Err(err);
                event_loop.exit();
                return;
            }
        };

        let gl_display = gl_config.display();
        let raw_window_handle = window.window_handle().ok().map(|wh| wh.as_raw());

        let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);
        let fallback_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::Gles(None))
            .build(raw_window_handle);

        let not_current_context = unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .unwrap_or_else(|_| {
                    gl_display
                        .create_context(&gl_config, &fallback_attributes)
                        .expect("no se pudo crear el contexto GL")
                })
        };

        let attrs = window
            .build_surface_attributes(Default::default())
            .expect("fallo al construir surface attributes");
        let gl_surface = unsafe {
            gl_display
                .create_window_surface(&gl_config, &attrs)
                .unwrap()
        };

        let gl_context = not_current_context.make_current(&gl_surface).unwrap();

        let gl = unsafe {
            glow::Context::from_loader_function(|s| {
                gl_display.get_proc_address(&CString::new(s).unwrap()) as *const _
            })
        };
        let renderer = Renderer::new(gl);

        if let Err(err) = gl_surface
            .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
        {
            eprintln!("No se pudo activar vsync: {err:?}");
        }

        self.state = Some(AppState {
            window,
            gl_context,
            gl_surface,
            renderer,
        });
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(state) = self.state.as_ref() else {
            return;
        };

        match event {
            WindowEvent::Resized(size) if size.width != 0 && size.height != 0 => {
                state.gl_surface.resize(
                    &state.gl_context,
                    NonZeroU32::new(size.width).unwrap(),
                    NonZeroU32::new(size.height).unwrap(),
                );
                state.renderer.resize(size.width as i32, size.height as i32);
            }
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape),
                        ..
                    },
                ..
            } => event_loop.exit(),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let Some(state) = self.state.as_ref() else {
            return;
        };
        state.renderer.draw();
        state.window.request_redraw();
        state.gl_surface.swap_buffers(&state.gl_context).unwrap();
    }
}

struct Renderer {
    program: glow::Program,
    vao: glow::VertexArray,
    vbo: glow::Buffer,
    gl: glow::Context,
}

impl Renderer {
    fn new(gl: glow::Context) -> Self {
        unsafe {
            let vertex_shader = create_shader(&gl, glow::VERTEX_SHADER, VERTEX_SHADER_SOURCE);
            let fragment_shader = create_shader(&gl, glow::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE);

            let program = gl.create_program().unwrap();
            gl.attach_shader(program, vertex_shader);
            gl.attach_shader(program, fragment_shader);
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_info_log(program));
            }
            gl.delete_shader(vertex_shader);
            gl.delete_shader(fragment_shader);
            gl.use_program(Some(program));

            let vao = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vao));

            let vbo = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

            let data: &[u8] = std::slice::from_raw_parts(
                VERTEX_DATA.as_ptr() as *const u8,
                std::mem::size_of_val(&VERTEX_DATA),
            );
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, data, glow::STATIC_DRAW);

            let stride = 5 * std::mem::size_of::<f32>() as i32;
            let pos_attrib = gl.get_attrib_location(program, "position").unwrap();
            let color_attrib = gl.get_attrib_location(program, "color").unwrap();

            gl.vertex_attrib_pointer_f32(pos_attrib, 2, glow::FLOAT, false, stride, 0);
            gl.vertex_attrib_pointer_f32(
                color_attrib,
                3,
                glow::FLOAT,
                false,
                stride,
                2 * std::mem::size_of::<f32>() as i32,
            );
            gl.enable_vertex_attrib_array(pos_attrib);
            gl.enable_vertex_attrib_array(color_attrib);

            Self {
                program,
                vao,
                vbo,
                gl,
            }
        }
    }

    fn draw(&self) {
        unsafe {
            self.gl.clear_color(0.1, 0.1, 0.1, 1.0);
            self.gl.clear(glow::COLOR_BUFFER_BIT);
            self.gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }
    }

    fn resize(&self, width: i32, height: i32) {
        unsafe { self.gl.viewport(0, 0, width, height) };
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_program(self.program);
            self.gl.delete_buffer(self.vbo);
            self.gl.delete_vertex_array(self.vao);
        }
    }
}

unsafe fn create_shader(gl: &glow::Context, kind: u32, source: &str) -> glow::Shader {
    let shader = gl.create_shader(kind).unwrap();
    gl.shader_source(shader, source);
    gl.compile_shader(shader);
    if !gl.get_shader_compile_status(shader) {
        panic!("{}", gl.get_shader_info_log(shader));
    }
    shader
}

#[rustfmt::skip]
static VERTEX_DATA: [f32; 15] = [
    -0.5, -0.5,  1.0, 0.0, 0.0,
     0.0,  0.5,  0.0, 1.0, 0.0,
     0.5, -0.5,  0.0, 0.0, 1.0,
];

const VERTEX_SHADER_SOURCE: &str = r#"
#version 100
precision mediump float;
attribute vec2 position;
attribute vec3 color;
varying vec3 v_color;
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_color = color;
}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 100
precision mediump float;
varying vec3 v_color;
void main() {
    gl_FragColor = vec4(v_color, 1.0);
}
"#;
