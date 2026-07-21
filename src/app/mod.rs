mod config;
mod event;
mod input;
mod run;
mod ui;

use std::error::Error;
use std::ffi::CString;
use std::num::NonZeroU32;

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
use crate::meshes::mesh::{MeshHandle, MeshLibrary};
use crate::renderer::{
    BillboardRenderer, OceanRenderer, SimpleColorRenderer, SkyboxRenderer, StandardRenderer,
};
use crate::scene::{Instance, Scene};
use crate::simulations::Simulation;
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

pub struct GraphicsContext {
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

    pub fn gl_and_libraries(&mut self) -> (&glow::Context, &mut MeshLibrary, &mut TextureLibrary) {
        let gl = self
            .renderer
            .gl_context()
            .map(|e| &*e)
            .expect("Glow Context no existente");
        (gl, &mut self.mesh_library, &mut self.texture_library)
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ClickMode {
    Insert,
    Select,
}

pub struct UiContext {
    pub imgui_ctx: dear_imgui_rs::Context,
    pub platform: dear_imgui_winit::WinitPlatform,
}

struct UiState {
    mesh_to_draw: MeshHandle,
    mesh_name: String,
    selected_instance: Option<usize>,
    selected_material: Material,
    buffered_color: glam::Vec4,
    wireframe_enabled: bool,
    lighting_enabled: bool,
    click_mode: ClickMode,
}

impl UiState {
    pub fn new(default_texture_id: u32, default_mesh_to_draw: MeshHandle) -> Self {
        Self {
            mesh_to_draw: default_mesh_to_draw,
            mesh_name: "cube".to_string(),
            selected_instance: None,
            selected_material: Material {
                color: vec4(1f32, 1f32, 1f32, 1f32),
                shininess: 200f32,
                texture_id: default_texture_id,
            },
            buffered_color: vec4(1f32, 0f32, 0f32, 0.6f32),
            wireframe_enabled: false,
            lighting_enabled: false,
            click_mode: ClickMode::Insert,
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
    skybox_renderer: SkyboxRenderer,
    ocean_renderer: OceanRenderer,
    graph_ctx: GraphicsContext,
    scene: Scene,
    input: InputState,
    camera: Camera,
    floor_gizmo: FloorGizmo,
    light: Light,
    simulation: Simulation,
    ui_ctx: UiContext,
    ui_state: UiState,
    time: f32,
}

struct App {
    state: Option<AppState>,
    exit_state: Result<(), Box<dyn Error>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.resumed_impl(event_loop);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        self.window_event_impl(event_loop, id, event);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.about_to_wait_impl(event_loop);
    }
}
