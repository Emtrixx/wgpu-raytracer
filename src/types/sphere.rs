use cgmath::Vector3;
use wgpu::util::DeviceExt;

pub struct Sphere {
    pub position: Vector3<f32>,
    pub radius: f32,
    pub material_id: u32,
}


#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SphereUniform {
    pub position_and_radius: [f32; 4],
    // pub position: [f32; 3],
    // pub _padding: u32,
    // pub radius: f32,
    // pub material_id: u32,
}

pub struct SphereStorage {
    pub spheres: Vec<SphereUniform>,
    pub sphere_count: u32,
}


pub(crate) struct SphereState {
    // TODO: add lifetime to objects (Do I need them here?)
    // pub objects: &Vec<Sphere>,
    // pub storage: SphereStorage,
    pub buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    uniforms: Vec<SphereUniform>,
}

impl SphereState {
    pub fn new(objects: &Vec<Sphere>, device: &wgpu::Device) -> Self {

        let mut sphere_uniforms: Vec<SphereUniform> = vec![];

        for sphere in objects {
            sphere_uniforms.push(SphereUniform {
                position_and_radius: [sphere.position.x, sphere.position.y, sphere.position.z, sphere.radius],
                // _padding: 0,
                // radius: sphere.radius,
                // material_id: sphere.material_id,
            });
        }

        println!("Sphere count: {:?}", sphere_uniforms);


        let storage_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sphere Storage Buffer"),
            contents: bytemuck::cast_slice(sphere_uniforms.as_slice()),
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
                label: Some("sphere_bind_group_layout"),
            });

        let object_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &object_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: storage_buffer.as_entire_binding(),
            }],
            label: Some("sphere_bind_group"),
        });

        Self {
            uniforms: sphere_uniforms,
            buffer: storage_buffer,
            bind_group_layout: object_bind_group_layout,
            bind_group: object_bind_group,
        }
    }
}