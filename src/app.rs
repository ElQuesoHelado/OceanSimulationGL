use std::error::Error;
use std::ffi::CString;
use std::num::NonZeroU32;
use std::rc::Rc;

use dear_imgui_winit::HiDpiMode;
use glam::{Vec3, Vec4, vec3, vec4};
use glow::HasContext;
use raw_window_handle::HasWindowHandle;
use winit::application::ApplicationHandler;
use winit::event::{KeyEvent, MouseButton, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{Key, KeyCode, NamedKey, PhysicalKey};
use winit::window::{Window, WindowId};

use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext, Version};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SwapInterval, WindowSurface};
use glutin_winit::{DisplayBuilder, GlWindow};

use crate::camera::Camera;
use crate::gizmo::FloorGizmo;
use crate::input_state::InputState;
use crate::light::Light;
use crate::material::Material;
use crate::mesh::{self, MeshId, MeshLibrary};
use crate::mops::Transform;
use crate::rain::Rain;
use crate::renderer::{BillboardRenderer, SimpleColorRenderer, StandardRenderer};
use crate::scene::{Instance, Scene};
use crate::texture::TextureLibrary;
use crate::{figures, scene};

pub fn run() -> Result<(), Box<dyn Error>> {
    let event_loop = winit::event_loop::EventLoop::new()?;
    let mut app = App {
        state: None,
        exit_state: Ok(()),
    };
    event_loop.run_app(&mut app)?;
    app.exit_state
}

pub struct GraphicsContext {
    pub imgui_ctx: dear_imgui_rs::Context,
    pub platform: dear_imgui_winit::WinitPlatform,
    pub renderer: dear_imgui_glow::GlowRenderer,
    pub mesh_library: MeshLibrary,
    pub texture_library: TextureLibrary,
}

impl GraphicsContext {
    pub fn gl(&self) -> &glow::Context {
        self.renderer
            .gl_context()
            .map(|e| &*e)
            .expect("Glow Context no existente")
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe { self.gl().viewport(0, 0, width, height) };
    }

    pub fn clear(&self) {
        unsafe {
            self.gl().clear_color(0.2, 0.2, 0.2, 1.0);
            self.gl()
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }
}

// Necesario separar en state, esto por comportamiento de winit
struct AppState {
    window: Window,
    gl_context: PossiblyCurrentContext,
    gl_surface: Surface<WindowSurface>,
    standard_renderer: StandardRenderer,
    billboard_renderer: BillboardRenderer,
    floor_giz_renderer: SimpleColorRenderer,
    graph_ctx: GraphicsContext,
    scene: Scene,
    input: InputState,
    camera: Camera,
    floor_gizmo: FloorGizmo,
    light: Light,
    rain: Rain,
}

impl AppState {
    pub fn process_input(&mut self) {
        let alt = self.input.key_pressed(KeyCode::AltLeft);
        let shift = self.input.key_pressed(KeyCode::ShiftLeft);
        let left_pressed = self.input.mouse_button_pressed(MouseButton::Left);
        let w = self.input.key_pressed(KeyCode::KeyW);
        let s = self.input.key_pressed(KeyCode::KeyS);
        let a = self.input.key_pressed(KeyCode::KeyA);
        let d = self.input.key_pressed(KeyCode::KeyD);

        if alt && left_pressed {
            self.camera.orbit(
                self.input.mouse_dx as f32 * 0.1,
                self.input.mouse_dy as f32 * 0.1,
            );
            return;
        } else if shift && left_pressed {
            self.camera
                .pan(self.input.mouse_dx as f32, self.input.mouse_dy as f32);
            return;
        }
        //TODO:
        if d {
            self.camera.target.x += 0.1;
        }
        if a {
            self.camera.target.x -= 0.1;
        }
    }
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
            .with_debug(true)
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

        let mut gl = unsafe {
            glow::Context::from_loader_function(|s| {
                gl_display.get_proc_address(&CString::new(s).unwrap()) as *const _
            })
        };

        unsafe {
            gl.enable(glow::DEBUG_OUTPUT);
            gl.enable(glow::DEBUG_OUTPUT_SYNCHRONOUS);
            gl.debug_message_callback(|_source, _typ, _id, _severity, message| {
                eprintln!("GL DEBUG: {}", message);
            });
        }

        if let Err(err) = gl_surface
            .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
        {
            eprintln!("No se pudo activar vsync: {err:?}");
        }

        let standard_renderer = StandardRenderer::new(
            &gl,
            "assets/shaders/shader.vert",
            "assets/shaders/shader.frag",
        )
        .expect("Creacion de Renderer Standard fallida");

        let billboard_renderer = BillboardRenderer::new(
            &gl,
            "assets/shaders/billboard.vert",
            "assets/shaders/billboard.frag",
        )
        .expect("Creacion de Renderer Billboard fallida");

        let floor_giz_renderer = SimpleColorRenderer::new(
            &gl,
            "assets/shaders/floor_giz.vert",
            "assets/shaders/floor_giz.frag",
        )
        .expect("Creacion de Renderer Piso Gizmo fallida");

        let floor_gizmo = FloorGizmo::new(&gl);

        let mesh_library = MeshLibrary::new(&gl);
        let texture_library = TextureLibrary::new(&gl, "assets/textures");

        let mut scene = Scene::new();

        let material = Material::new(&texture_library, vec4(1., 1., 1., 1.), 32., "hinojosa")
            .expect("Textura no encontrada"); //TODO: en UI no hacer panic?

        scene.add_prim_instance(Instance::new(MeshId::Cube, material));

        let size = window.inner_size();
        let camera = Camera::new(size.width as f32 / size.height as f32);

        let light = Light {
            enabled: true,
            pos: vec3(1f32, 1f32, 1f32),
            color: vec3(1f32, 1f32, 1f32),
        };

        let rain = Rain::new(&mut scene, &texture_library, 2000);

        let mut imgui_ctx = dear_imgui_rs::Context::create();
        let mut platform = dear_imgui_winit::WinitPlatform::new(&mut imgui_ctx);
        platform.attach_window(&window, HiDpiMode::Default, &mut imgui_ctx);

        let mut renderer = dear_imgui_glow::GlowRenderer::new(gl, &mut imgui_ctx)
            .expect("Creacion de renderer Imgui fallida");

        self.state = Some(AppState {
            window,
            gl_context,
            gl_surface,
            standard_renderer,
            billboard_renderer,
            floor_giz_renderer,
            graph_ctx: GraphicsContext {
                imgui_ctx,
                platform,
                renderer,
                mesh_library,
                texture_library,
            },
            scene,
            input: InputState::default(),
            camera,
            floor_gizmo,
            light,
            rain,
        });
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(state) = self.state.as_mut() else {
            return;
        };

        match event {
            WindowEvent::CursorMoved { position, .. } => {
                state.input.on_cursor_moved(position.x, position.y);
            }
            WindowEvent::MouseInput {
                button,
                state: btn_state,
                ..
            } => {
                state.input.on_mouse_button(button, btn_state);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                use winit::keyboard::{KeyCode, PhysicalKey};

                if let PhysicalKey::Code(code) = event.physical_key {
                    if code == KeyCode::Escape {
                        event_loop.exit();
                    }

                    state.input.on_keyboard_input(code, event.state);
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

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let Some(state) = self.state.as_mut() else {
            return;
        };

        state.rain.update(&mut state.scene, 0.11f32);

        state.process_input();
        state.input.end_frame();

        state.graph_ctx.clear();
        state.standard_renderer.draw(
            &state.graph_ctx,
            &state.scene.prim_instances,
            &state.camera,
            &state.light,
        );

        state
            .billboard_renderer
            .draw(&state.graph_ctx, &state.scene.bill_instances, &state.camera);

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
