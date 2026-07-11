use super::*;

impl App {
    pub fn get_gl_context(&self) -> Option<&glow::Context> {
        self.state
            .as_ref()?
            .graph_ctx
            .renderer
            .gl_context()
            .map(|rc| rc.as_ref())
    }

    pub fn resumed_impl(&mut self, event_loop: &ActiveEventLoop) {
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

        // let material = Material::new(&texture_library, vec4(1., 1., 1., 1.), 32., "hinojosa")
        //     .expect("Textura no encontrada"); //TODO: en UI no hacer panic?

        //scene.add_normal_instance(Instance::new(MeshId::Cube, material));

        let size = window.inner_size();
        let camera = Camera::new(size.width as f32 / size.height as f32);

        let light = Light {
            enabled: true,
            pos: vec3(1f32, 1f32, 1f32),
            color: vec3(1f32, 1f32, 1f32),
        };

        let rain = Rain::new(&mut scene, &texture_library, 2000);

        // Init Imgui
        let mut imgui_ctx = dear_imgui_rs::Context::create();
        let mut platform = dear_imgui_winit::WinitPlatform::new(&mut imgui_ctx);
        platform.attach_window(&window, HiDpiMode::Default, &mut imgui_ctx);

        let renderer = dear_imgui_glow::GlowRenderer::new(gl, &mut imgui_ctx)
            .expect("Creacion de renderer Imgui fallida");

        let blank_tex_id = texture_library
            .get_id_from_name("blank")
            .expect("No existe textura default(blank)");

        self.state = Some(AppState {
            window,
            gl_context,
            gl_surface,
            standard_renderer,
            billboard_renderer,
            floor_giz_renderer,
            graph_ctx: GraphicsContext {
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
            ui_ctx: UiContext { imgui_ctx },
            ui_state: UiState::new(blank_tex_id),
        });
    }
}
