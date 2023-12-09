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
    pub metadata_buffer: wgpu::Buffer,
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

        Self {
            buffer: material_buffer,
            metadata_buffer: material_metadata_buffer,
            uniforms: material_uniforms,
        }
    }
}