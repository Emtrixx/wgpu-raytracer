use crate::camera::camera_state::CameraState;
use crate::pipelines::compute_pipeline::create_compute_pipeline;
use crate::pipelines::render_pipeline::create_render_pipeline;
use types::vertex;
use wgpu::util::DeviceExt;
use wgpu::{Buffer, Device, Gles3MinorVersion};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use crate::types::globals::GlobalState;
use crate::types::material::{Material, MaterialState};

mod camera;
mod pipelines;
mod types;

struct State {
    surface: wgpu::Surface,
    device: Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,
    // Globals
    global_state: GlobalState,
    // Materials
    material_state: MaterialState,
    // Spheres
    sphere_state: types::sphere::SphereState,
    // Camera
    camera_state: CameraState,
    // Texture and Sampler
    vertex_buffer: Buffer,
    // Raytracing
    rt_pipeline: wgpu::ComputePipeline,
    rt_bind_group: wgpu::BindGroup,
    // Rendering
    render_pipeline: wgpu::RenderPipeline,
    render_bind_group: wgpu::BindGroup,
}

impl State {
    async fn new(window: Window) -> Self {
        // Basic config

        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            // backends: wgpu::Backends::GL,
            dx12_shader_compiler: Default::default(),
            flags: wgpu::InstanceFlags::default(),
            gles_minor_version: Gles3MinorVersion::Version0,
        });

        // List adapters
        // instance
        //     .enumerate_adapters(wgpu::Backends::all())
        //     .for_each(|adapter| {
        //         println!("Adapter: {:?}", adapter.get_info());
        //     });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                // compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::union(
                        wgpu::Features::default(),
                        wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES, // | wgpu::Features::STORAGE_RESOURCE_BINDING_ARRAY,
                    ),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        // let surface_format = surface_caps
        //     .formats
        //     .iter()
        //     .copied()
        //     .find(|f| f.is_srgb())
        //     .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8Unorm,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);


        // Texture and Sampler
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: size.width,
                height: size.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1, // We'll talk about this a little later
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            // Most images are stored using sRGB so we need to reflect that here.
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::STORAGE_BINDING,
            // | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: Some("diffuse_texture"),
            view_formats: &[],
        });

        let sampler_desc = wgpu::SamplerDescriptor {
            label: Some("raytracing sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
        };

        let sampler = device.create_sampler(&sampler_desc);

        let rt_texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
            format: Some(wgpu::TextureFormat::Rgba8Unorm),
            ..Default::default()
        });

        // Vertex buffer for quad surface
        let vertex_buffer = Self::create_vertex_buffer(&device);

        // Globals
        let global_state = GlobalState::new(&device);

        // Materials
        let materials = vec![
            Material {
                color: [0.8, 0.0, 0.0],
                emission_color: [0.0, 0.0, 0.0],
                emission_strength: 0.0,
            },
            Material {
                color: [1.0, 1.0, 1.0],
                emission_color: [1.0, 1.0, 1.0],
                emission_strength: 1.0,
            },
            Material {
                color: [0.0, 0.0, 0.8],
                emission_color: [0.0, 0.0, 0.0],
                emission_strength: 0.0,
            },
            // ground
            Material {
                color: [0.4, 0.4, 0.4],
                emission_color: [0.0, 0.0, 0.0],
                emission_strength: 0.0,
            },
        ];
        let material_state = MaterialState::new(&materials, &device);

        // Spheres
        let spheres = vec![
            types::sphere::Sphere {
                position: cgmath::Vector3::new(-3.0, 0.0, 0.0),
                radius: 1.0,
                material_id: 0,
            },
            types::sphere::Sphere {
                position: cgmath::Vector3::new(0.0, 150.0, 100.0),
                radius: 100.0,
                material_id: 1,
            },
            types::sphere::Sphere {
                position: cgmath::Vector3::new(3.0, 0.0, 0.0),
                radius: 1.0,
                material_id: 2,
            },
            types::sphere::Sphere {
                position: cgmath::Vector3::new(0.0, -51.0, 3.0),
                radius: 50.0,
                material_id: 3,
            },
        ];

        let sphere_state = types::sphere::SphereState::new(&spheres, &device);

        // Camera
        let camera_state = CameraState::new(&device, &config);

        // Raytracing
        let (rt_pipeline, rt_bind_group) = create_compute_pipeline(
            &device,
            &rt_texture_view,
            &global_state,
            &camera_state,
            &sphere_state,
            &material_state,
        );

        // Rendering
        let render_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Render Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        count: None,
                    },
                ],
            });

        let render_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Render Bind Group"),
            layout: &render_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&rt_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        let render_pipeline = create_render_pipeline(&config, &device, &render_bind_group_layout);

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            vertex_buffer,
            global_state,
            material_state,
            sphere_state,
            camera_state,
            rt_pipeline,
            rt_bind_group,
            render_pipeline,
            render_bind_group,
        }
    }

    fn create_vertex_buffer(device: &Device) -> Buffer {
        // Quad

        let vertices = [
            vertex::Vertex {
                position: [1.0, 1.0],
                tex_coords: [1.0, 0.0],
            },
            vertex::Vertex {
                position: [-1.0, 1.0],
                tex_coords: [0.0, 0.0],
            },
            vertex::Vertex {
                position: [-1.0, -1.0],
                tex_coords: [0.0, 1.0],
            },
            vertex::Vertex {
                position: [1.0, 1.0],
                tex_coords: [1.0, 0.0],
            },
            vertex::Vertex {
                position: [-1.0, -1.0],
                tex_coords: [0.0, 1.0],
            },
            vertex::Vertex {
                position: [1.0, -1.0],
                tex_coords: [1.0, 1.0],
            },
        ];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        vertex_buffer
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.height = new_size.height;
            self.config.width = new_size.height;
            self.surface.configure(&self.device, &self.config)
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        self.camera_state.controller.process_events(event)
    }

    fn update(&mut self) {
        // update global state
        self.global_state.update(&self.queue);
        // update camera
        self.camera_state.update(&self.queue);
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("My fancy compute encoder"),
            });

        // compute
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("My fancy compute pass"),
                timestamp_writes: None,
            });

            cpass.set_pipeline(&self.rt_pipeline);
            cpass.set_bind_group(0, &self.rt_bind_group, &[]);
            cpass.dispatch_workgroups(self.config.width, self.config.height, 1);
        }

        // render
        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline); // 2.
            render_pass.set_bind_group(0, &self.render_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..6, 0..1);
        }
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(window).await;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == state.window.id() => {
            if !state.input(event) {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
        }
        Event::RedrawRequested(window_id) if window_id == state.window().id() => {
            state.update();
            match state.render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            state.window().request_redraw();
        }
        _ => {}
    });
}
