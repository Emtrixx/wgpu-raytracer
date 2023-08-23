use wgpu::{BindGroup, BindGroupLayout, Buffer};
use wgpu::util::DeviceExt;
use crate::camera::camera_controller::CameraController;
use crate::camera::main_camera::{Camera, CameraUniform};

pub struct CameraState {
    pub(crate) object: Camera,
    pub(crate) uniform: CameraUniform,
    pub(crate) rotation_buffer: Buffer,
    pub(crate) eye_buffer: Buffer,
    pub(crate) bind_group: BindGroup,
    pub(crate) controller: CameraController,
    pub(crate) bind_group_layout: BindGroupLayout,
}

impl CameraState {
    pub(crate) fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let object = Camera {
            // position the camera one unit up and 2 units back
            // +z is out of the screen
            eye: (0.0, 0.0, -5.0).into(),
            // have it look at the origin
            target: (0.0, 0.0, 0.0).into(),
            // which way is "up"
            up: cgmath::Vector3::unit_y(),
            aspect: config.width as f32 / config.height as f32,
            fovy: 70.0,
            znear: 0.01,
        };

        let mut uniform = CameraUniform::new();
        uniform.update(&object);

        let rotation_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[uniform.rotation_matrix]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let eye_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Eye Buffer"),
            contents: bytemuck::cast_slice(&[uniform.eye]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // let view_params

        let bind_group_layout =
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
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                label: Some("camera_bind_group_layout"),
            });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: rotation_buffer.as_entire_binding(),
            }, wgpu::BindGroupEntry {
                binding: 1,
                resource: eye_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let controller = CameraController::new(0.2);

        Self {
            bind_group,
            controller,
            rotation_buffer,
            eye_buffer,
            uniform,
            object,
            bind_group_layout,
        }
    }
}