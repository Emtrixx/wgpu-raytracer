use wgpu::util::DeviceExt;

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::core::mem::size_of::<T>(),
    )
}

pub struct Material {
    pub color: [f32; 3],
    pub emission_color: [f32; 3],
    pub emission_strength: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MaterialUniform {
    pub color: [f32; 3],
    pub _padding: u32,
    pub emission_color: [f32; 3],
    pub emission_strength: f32,
}

pub struct MaterialState {
    pub buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    pub uniforms: Vec<MaterialUniform>,
}

impl MaterialState {
    pub fn new(materials: &Vec<Material>, device: &wgpu::Device) -> MaterialState {
        let material_uniforms: Vec<MaterialUniform> = materials.iter().map(|material| {
            MaterialUniform {
                color: material.color,
                _padding: 0,
                emission_color: material.emission_color,
                emission_strength: material.emission_strength,
            }
        }).collect();

        let material_metadata_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Material Metadata Buffer"),
            contents: bytemuck::cast_slice(&[material_uniforms.len() as u32]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let material_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Material Storage Buffer"),
            contents: bytemuck::cast_slice(material_uniforms.as_slice()),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let object_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                label: Some("material_bind_group_layout"),
            });

        let object_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &object_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: material_metadata_buffer.as_entire_binding(),
            },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: material_buffer.as_entire_binding(),
                }],
            label: Some("material_bind_group"),
        });

        Self {
            buffer: material_buffer,
            bind_group_layout: object_bind_group_layout,
            bind_group: object_bind_group,
            uniforms: material_uniforms,
        }
    }
}