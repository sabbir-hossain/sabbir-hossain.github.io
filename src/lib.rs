use std::{iter, sync::Arc};
use std::rc::Rc;
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub mod components;
pub mod scene;
pub mod option;
// pub mod text;
pub mod utils;
pub mod text;

use option::{ViewConfig, ViewObject, Scene};
use utils::vertex::Vertex;
use utils::draw;

use crate::utils::color::BACKGROUND_COLOR;
use crate::utils::draw::TxUniform;
// use crate::utils::sound;

pub struct State {
    surface: wgpu::Surface<'static>,
    pub device: Rc<wgpu::Device>,
    pub queue: Rc<wgpu::Queue>,
    config: wgpu::SurfaceConfiguration,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
    is_surface_configured: bool,
    window: Arc<Window>,

    render_pipeline: wgpu::RenderPipeline,

    pub view_config: Rc<ViewConfig>,
    pub scene: Scene,
    pub init_scene: scene::scene0::Scene0,
    pub objects: Vec<ViewObject>,
}

impl State {
    async fn new(window: Arc<Window>, init_size: (u32, u32)) -> anyhow::Result<State> {
        // let size = window.inner_size();
        // log::info!("Creating wgpu state with size: {:?}", size);
        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);

        // Shader code in this tutorial assumes an Srgb surface texture. Using a different
        // one will result all the colors comming out darker. If you want to support non
        // Srgb surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: init_size.0,
            height: init_size.1,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };

        let view_config = Rc::new(ViewConfig::new(config.width, config.height));
        log::info!("ViewConfig: {:?}", view_config);
        
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
                label: Some("bind_group_layout"),
            });
 
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &uniform_bind_group_layout,
                    &texture_bind_group_layout, 
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),

            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                // or Features::POLYGON_MODE_POINT
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // Uncomment if you want to use depth/stencil
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
            // Useful for optimizing shader compilation on Android
            cache: None,
        });

        let init_scene = scene::scene0::Scene0::new(view_config.clone());

        let mut scene = Scene::Scene0(init_scene.clone());
        let objects: Vec<ViewObject> = scene.draw();


        Ok(Self {
            surface,
            device: Rc::new(device),
            queue: Rc::new(queue),
            config,
            is_surface_configured: false,
            window,
            texture_bind_group_layout,
            uniform_bind_group_layout,
            render_pipeline,

            view_config: Rc::clone(&view_config),
            init_scene: init_scene.clone(),
            scene,
            // scene: Scene::Scene0(init_scene),
            // init_scene,
            // scene: Scene::Scene3(scene),
            objects,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
    }

    fn update(&mut self) {
        
        let objects = self.scene.update_scene(); // Capture the returned Vec<ViewObject>
        self.objects = objects.iter().map(|obj| obj.clone()).collect(); 

        for object in &self.objects {
            let (uniform_buffer, tex_buffer)= draw::transform_draw_object(
                Rc::clone(&self.device),
                object.clone(),
            ).expect("Failed to transform draw object");

            let mvp_ref: &[f32; 16] = object.model_matrix.as_ref();
            self.queue.write_buffer(
                &uniform_buffer, 
                0, 
                bytemuck::cast_slice(mvp_ref)
            );

            let tx_uniform = TxUniform {
                offset: object.texture_offset.into(),
                angle: object.tx_rotation,
                _padding: 0.0
                // _padding: [0.0; 2],
            };

            self.queue.write_buffer(
                &tex_buffer, 
                0, 
                bytemuck::bytes_of(&tx_uniform)
            );
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();
        
        // We can't render unless the surface is configured
        if !self.is_surface_configured {
            return Ok(());
        }

        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: BACKGROUND_COLOR[0] as f64,
                            g: BACKGROUND_COLOR[1] as f64,
                            b: BACKGROUND_COLOR[2] as f64,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);

            for object in &self.objects {
                let draw_object: draw::DrawObject = draw::generate_draw_object(
                    Rc::clone(&self.device),
                    Rc::clone(&self.queue),
                    &self.uniform_bind_group_layout,
                    &self.texture_bind_group_layout,
                    object.clone(),
                ).expect("Failed to generate draw object");

                render_pass.set_bind_group(0, &draw_object.uniform_bind_group, &[]);              
                render_pass.set_bind_group(1, &draw_object.texture_bind_group, &[]);

                render_pass.set_vertex_buffer(0, draw_object.vertex_buffer.slice(..));
                render_pass.set_index_buffer(draw_object.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..object.indices_len, 0, 0..1);
            } 
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
    
    fn handle_key(&mut self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        // handle_keyboard_input
        // log::info!("Key pressed: {:?}, is_pressed: {}", code, is_pressed);
        if code == KeyCode::Escape && is_pressed {
            log::info!("Escape key pressed, exiting the application");
            event_loop.exit();
            return;
        }
        else if (code == KeyCode::F1 || code == KeyCode::Home) && is_pressed {
            // sound::select_init_scene();
            self.objects = self.init_scene.draw();
            self.scene = Scene::Scene0(self.init_scene.clone());
            log::info!("F1 key pressed, returning to Scene0");
        } else {
            if is_pressed {
                let scene = self.scene.handle_keyboard_input(code);
                if let Some(mut scene) = scene {
                    scene.draw();
                    self.scene = scene;
                } else {
                    log::info!("Scene is None");
                }
            }
        }
        
        // match (code, is_pressed) {
        //     (KeyCode::Escape, true) => event_loop.exit(),
        //     other => self.scene.handle_keyboard_input(other.0),
        // }
    }
}

pub struct App {
    #[cfg(target_arch = "wasm32")]
    proxy: Option<winit::event_loop::EventLoopProxy<State>>,
    state: Option<State>,
}

impl App {
    pub fn new(#[cfg(target_arch = "wasm32")] event_loop: &EventLoop<State>) -> Self {
        #[cfg(target_arch = "wasm32")]
        let proxy = Some(event_loop.create_proxy());
        Self {
            state: None,
            #[cfg(target_arch = "wasm32")]
            proxy,
        }
    }
}

impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

         #[allow(unused_assignments)]
        let mut size = (0, 0);
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowAttributesExtWebSys;

            const CANVAS_ID: &str = "wasm";

            let window = wgpu::web_sys::window().unwrap_throw();
            let document = window.document().unwrap_throw();
            let canvas = document.get_element_by_id(CANVAS_ID).unwrap_throw();
            let html_canvas_element: wgpu::web_sys::HtmlCanvasElement = canvas.unchecked_into();

            // Get CSS-rendered size
            let rect = html_canvas_element.get_bounding_client_rect();
            let css_width = rect.width();
            let css_height = rect.height();

            // Set actual pixel buffer size
            html_canvas_element.set_width(css_width as u32);
            html_canvas_element.set_height(css_height as u32);

            // Now .width() and .height() return correct pixel size
            let width = html_canvas_element.width();
            let height = html_canvas_element.height();

            size = (width, height);

            window_attributes = window_attributes.with_canvas(Some(html_canvas_element));
        }

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        #[cfg(not(target_arch = "wasm32"))]
        {
            // If we are not on web we can use pollster to
            // await the
            self.state = Some(pollster::block_on(State::new(window, size)).unwrap());
        }

        #[cfg(target_arch = "wasm32")]
        {
            if let Some(proxy) = self.proxy.take() {
                wasm_bindgen_futures::spawn_local(async move {
                    assert!(proxy
                        .send_event(
                            State::new(window, size)
                                .await
                                .expect("Unable to create canvas!!!")
                        )
                        .is_ok())
                });
            }
        }
    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: State) {
        #[cfg(target_arch = "wasm32")]
        {
            event.window.request_redraw();
            event.resize(
                event.window.inner_size().width,
                event.window.inner_size().height,
            );
        }
        self.state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = state.window.inner_size();
                        state.resize(size.width, size.height);
                    }
                    Err(e) => {
                        log::error!("Unable to render {}", e);
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => match (button, state.is_pressed()) {
                (MouseButton::Left, true) => {}
                (MouseButton::Left, false) => {}
                _ => {}
            },
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => state.handle_key(event_loop, code, key_state.is_pressed()),
            _ => {}
        }
    }
}

pub fn run() -> anyhow::Result<()> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
    }
    #[cfg(target_arch = "wasm32")]
    {
        console_log::init_with_level(log::Level::Info).unwrap_throw();
    }

    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new(
        #[cfg(target_arch = "wasm32")]
        &event_loop,
    );
    event_loop.run_app(&mut app)?;

    Ok(())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    run().unwrap_throw();

    Ok(())
}
