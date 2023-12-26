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
    // pub position_and_radius: [f32; 4],
    pub position: [f32; 3],
    pub radius: f32,
    pub material_id: u32,
    pub _padding: [u32; 3],
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
    pub metadata_buffer: wgpu::Buffer,
    pub uniforms: Vec<SphereUniform>,
}

impl SphereState {
    pub fn new(objects: &Vec<Sphere>, device: &wgpu::Device) -> Self {
        let mut sphere_uniforms: Vec<SphereUniform> = vec![];

        for sphere in objects {
            sphere_uniforms.push(SphereUniform {
                // position_and_radius: [sphere.position.x, sphere.position.y, sphere.position.z, sphere.radius],
                position: sphere.position.into(),
                radius: sphere.radius,
                material_id: sphere.material_id,
                _padding: [0, 0, 0],
            });
        }

        println!("Sphere count: {:?}", sphere_uniforms);

        let sphere_metadata_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sphere Metadata Buffer"),
            contents: bytemuck::cast_slice(&[sphere_uniforms.len() as u32]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let storage_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sphere Storage Buffer"),
            contents: bytemuck::cast_slice(sphere_uniforms.as_slice()),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            uniforms: sphere_uniforms,
            buffer: storage_buffer,
            metadata_buffer: sphere_metadata_buffer,
        }
    }
}
