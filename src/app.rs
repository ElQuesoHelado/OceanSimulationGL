use std::error::Error;
use std::ffi::CString;
use std::num::NonZeroU32;

use raw_window_handle::HasWindowHandle;
use winit::application::ApplicationHandler;
use winit::event::{KeyEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowId};

use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext, Version};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SwapInterval, WindowSurface};
use glutin_winit::{DisplayBuilder, GlWindow};

use crate::camera::Camera;
use crate::figures;
use crate::gizmo::Gizmo;
use crate::mesh::{self, MeshLibrary};
use crate::renderer::StandardRenderer;
use crate::scene::{Instance, Scene};
use crate::texture::TextureLibrary;

pub fn run() -> Result<(), Box<dyn Error>> {
    let event_loop = winit::event_loop::EventLoop::new()?;
    let mut app = App {
        state: None,
        exit_state: Ok(()),
    };
    event_loop.run_app(&mut app)?;
    app.exit_state
}

//Necesario separar en state, esto por comportamiento de winit
struct AppState {
    window: Window,
    gl_context: PossiblyCurrentContext,
    gl_surface: Surface<WindowSurface>,
    standard_renderer: StandardRenderer,
    scene: Scene,
    camera: Camera,
    gizmo: Gizmo,
    texture_library: TextureLibrary,
}

struct App {
    state: Option<AppState>,
    exit_state: Result<(), Box<dyn Error>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_some() {
            return;
        }

        let window_attributes =
            Window::default_attributes().with_title("Editor 3D (Escape para salir)");

        let template = ConfigTemplateBuilder::new();
        let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes));

        // Aparentemente se tiene que buscar todas las "configs"
        // hasta conseguir una(max antialising samples)
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

        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(4, 6))))
            .build(raw_window_handle);

        let not_current_context = unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .expect("no se pudo crear un contexto OpenGL 4.6 — revisa drivers/GPU")
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

        if let Err(err) = gl_surface
            .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
        {
            eprintln!("No se pudo activar vsync: {err:?}");
        }

        let mut renderer = StandardRenderer::new(
            &gl,
            "assets/shaders/shader.vert",
            "assets/shaders/shader.vert",
        );
        // let gizmo = Gizmo::new(&renderer.gl);

        // --- Escena de ejemplo: un cubo con material de color plano ---
        let mut mesh_library: MeshLibrary;
        mesh_library.add(&gl, &figures::cube());

        let mut scene = Scene::new();
        let idx = scene.add(Instance {
            transform: glam::Mat4::IDENTITY,
            mesh: cube_mesh,
            material: red_material,
        });
        scene.selected = Some(idx);

        let camera = Camera::new();

        self.state = Some(AppState {
            window,
            gl_context,
            gl_surface,
            renderer,
            scene,
            camera,
            gizmo,
        });
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(state) = self.state.as_mut() else {
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
                state
                    .camera
                    .set_aspect(size.width as f32, size.height as f32);
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

        state.renderer.clear();
        state.renderer.draw_scene(&state.scene, &state.camera);

        if let Some(idx) = state.scene.selected {
            let world_pos = state.scene.instances[idx].transform.w_axis.truncate();
            state
                .gizmo
                .draw(&state.renderer.gl, &state.camera, world_pos);
        }

        state.window.request_redraw();
        state.gl_surface.swap_buffers(&state.gl_context).unwrap();
    }
}
