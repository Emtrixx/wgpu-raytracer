// TODO: Generic buffer for all intersectable types possible? Sending generic data to the GPU?

use wgpu::util::DeviceExt;

struct IntersectableContainer {
    pub object: Box<dyn Intersectable>,
    pub intersectable_type: IntersectableType,
    pub material_id: u32,
}

pub enum IntersectableType {
    Sphere,
}

pub trait Intersectable {}

struct IntersectableState {
    pub objects: Vec<IntersectableContainer>,
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

impl IntersectableState {
    pub fn new(objects: Vec<IntersectableContainer>, device: &wgpu::Device) -> Self {
        // let mut storage_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        //     label: Some("Intersectable Storage Buffer"),
        //     size: 0,
        //     usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        //     mapped_at_creation: false,
        // });
        let storage_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Intersectable Storage Buffer"),
            contents: bytemuck::cast_slice(&objects),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let object_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("object_bind_group_layout"),
            });

        let object_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &object_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: storage_buffer.as_entire_binding(),
            }],
            label: Some("object_bind_group"),
        });

        Self {
            objects,
            buffer: storage_buffer,
            bind_group: object_bind_group,
        }
    }
}
