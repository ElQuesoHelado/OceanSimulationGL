use crate::{scene::Skybox, simulations::ocean::Ocean};

use super::*;

impl App {
    pub fn resumed_impl(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_some() {
            return;
        }

        //*************************
        // Winit ,GL
        //*************************

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
            gl.enable(glow::DEPTH_TEST);
        }

        if let Err(err) = gl_surface
            .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
        {
            eprintln!("No se pudo activar vsync: {err:?}");
        }
        //*************************
        // Mesh/Mesh Handling related
        //*************************
        let mesh_library = MeshLibrary::new(&gl);
        let texture_library = TextureLibrary::new(&gl, "assets/textures");

        let mut scene = Scene::new();

        let size = window.inner_size();
        let camera = Camera::new(size.width as f32 / size.height as f32);

        let light = Light {
            enabled: true,
            pos: vec3(150f32, 50f32, 150f32),
            color: vec3(1f32, 1f32, 1f32),
        };

        let skybox = Skybox::new(&texture_library, &mesh_library, "NissiBeach");
        scene.set_skybox_instance(skybox);

        //*************************
        // Simulation
        //*************************

        let simulation = Simulation::new(&mut scene, &texture_library, &mesh_library);

        //*************************
        // Renderers
        //*************************

        let standard_renderer = StandardRenderer::new(
            &gl,
            "assets/shaders/normal_shape.vert",
            "assets/shaders/normal_shape.frag",
        )
        .expect("Creacion de Renderer Standard fallida");

        let ocean_renderer = OceanRenderer::new(
            &gl,
            "assets/shaders/ocean.vert",
            "assets/shaders/normal_shape.frag",
            &simulation.ocean.waves,
            &mut scene,
            &texture_library,
            &mesh_library,
        )
        .expect("Creacion de Renderer Ocean fallida");

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

        let skybox_renderer = SkyboxRenderer::new(
            &gl,
            "assets/shaders/skybox.vert",
            "assets/shaders/skybox.frag",
        )
        .expect("Creacion de Renderer Skybox fallida");

        let floor_gizmo = FloorGizmo::new(&gl);

        //*************************
        // Init Imgui
        //*************************

        let mut imgui_ctx = dear_imgui_rs::Context::create();
        let mut platform = dear_imgui_winit::WinitPlatform::new(&mut imgui_ctx);
        platform.attach_window(&window, HiDpiMode::Default, &mut imgui_ctx);

        let renderer = dear_imgui_glow::GlowRenderer::new(gl, &mut imgui_ctx)
            .expect("Creacion de renderer Imgui fallida");

        let blank_tex_id = texture_library
            .get_id_from_name("blank")
            .expect("No existe textura default(blank)");

        let cube_mesh_id = mesh_library
            .get_handle_from_name("cube")
            .expect("No existe mesh default(cube)");

        self.state = Some(AppState {
            window,
            gl_context,
            gl_surface,
            standard_renderer,
            ocean_renderer,
            billboard_renderer,
            floor_giz_renderer,
            skybox_renderer,
            graph_ctx: GraphicsContext {
                renderer,
                mesh_library,
                texture_library,
            },
            scene,
            input: InputState::default(),
            camera,
            floor_gizmo,
            light,
            simulation,
            ui_ctx: UiContext {
                imgui_ctx,
                platform,
            },
            ui_state: UiState::new(blank_tex_id, cube_mesh_id),
            time: 0.0,
        });
    }
}
