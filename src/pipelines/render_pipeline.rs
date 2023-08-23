use crate::types::vertex;

/// Creates a new render pipeline with the given configuration, device, and render bind group layout.
///
/// # Arguments
///
/// * `config` - A reference to a `wgpu::SurfaceConfiguration` object that specifies the configuration of the surface.
/// * `device` - A reference to a `wgpu::Device` object that represents the GPU device.
/// * `render_bind_group_layout` - A reference to a `wgpu::BindGroupLayout` object that represents the layout of the bind group used for rendering.
///
/// # Returns
///
/// A `wgpu::RenderPipeline` object that represents the newly created render pipeline.
///
/// # Example
///
/// ```
/// let render_pipeline = create_render_pipeline(&config, &device, &render_bind_group_layout);
/// ```
pub fn create_render_pipeline(
    config: &wgpu::SurfaceConfiguration,
    device: &wgpu::Device,
    render_bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../shader/shader.wgsl").into()),
    });

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[&render_bind_group_layout],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main", // 1.
            buffers: &[vertex::Vertex::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    });

    render_pipeline
}
