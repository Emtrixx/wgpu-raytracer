pub fn create_compute_pipeline(
    device: &wgpu::Device,
    rt_bind_group_layout: &wgpu::BindGroupLayout,
    camera_bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::ComputePipeline
{
    let rt_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("My fancy compute pipeline layout"),
        bind_group_layouts: &[rt_bind_group_layout, camera_bind_group_layout],
        push_constant_ranges: &[],
    });

    let rt_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("My fancy compute pipeline"),
        layout: Some(&rt_pipeline_layout),
        module: &device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("My fancy compute shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shader/raytracer.wgsl").into()),
        }),
        entry_point: "main",
    });

    rt_pipeline
}