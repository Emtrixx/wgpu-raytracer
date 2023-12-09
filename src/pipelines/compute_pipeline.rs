use wgpu::{BindGroup, ComputePipeline};

use crate::camera::camera_state::CameraState;
use crate::types::globals::GlobalState;
use crate::types::material::MaterialState;
use crate::types::sphere::SphereState;

/// Creates a compute pipeline for the raytracer using the given device, raytracer bind group layout, and camera bind group layout.
///
/// # Arguments
///
/// * `device` - A reference to the `wgpu::Device` to use for creating the pipeline.
/// * `rt_bind_group_layout` - A reference to the `wgpu::BindGroupLayout` for the raytracer.
/// * `camera_bind_group_layout` - A reference to the `wgpu::BindGroupLayout` for the camera.
///
/// # Returns
///
/// The created `wgpu::ComputePipeline`.
pub fn create_compute_pipeline(
    device: &wgpu::Device,
    rt_texture_view: &wgpu::TextureView,
    global_state: &GlobalState,
    camera_state: &CameraState,
    sphere_state: &SphereState,
    material_state: &MaterialState,
) -> (ComputePipeline, BindGroup) {

    // Bind Group
    let rt_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("My fancy compute bindings"),
            entries: &[
                // Storage texture binding for the raytracer output
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        view_dimension: wgpu::TextureViewDimension::D2,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        access: wgpu::StorageTextureAccess::WriteOnly,
                    },
                    count: None,
                },
                // Globals
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                //Camera
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Spheres
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Material
                wgpu::BindGroupLayoutEntry {
                    binding: 5,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 6,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
        });

    let rt_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("My fancy compute bind group"),
        layout: &rt_bind_group_layout,
        entries: &[
            // Binding 0: raytracer output
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(rt_texture_view),
            },
            // Binding 1: globals
            wgpu::BindGroupEntry {
                binding: 1,
                resource: global_state.buffer.as_entire_binding(),
            },
            // Binding 2: camera
            wgpu::BindGroupEntry {
                binding: 2,
                resource: camera_state.buffer.as_entire_binding(),
            },
            // Binding 3: sphere metadata
            wgpu::BindGroupEntry {
                binding: 3,
                resource: sphere_state.metadata_buffer.as_entire_binding(),
            },
            // Binding 4: sphere buffer
            wgpu::BindGroupEntry {
                binding: 4,
                resource: sphere_state.buffer.as_entire_binding(),
            },
            // Binding 5: material metadata
            wgpu::BindGroupEntry {
                binding: 5,
                resource: material_state.metadata_buffer.as_entire_binding(),
            },
            // Binding 6: material buffer
            wgpu::BindGroupEntry {
                binding: 6,
                resource: material_state.buffer.as_entire_binding(),
            }
        ],
    });

    let rt_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("My fancy compute pipeline layout"),
        bind_group_layouts: &[&rt_bind_group_layout],
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

    (rt_pipeline, rt_bind_group)
}
