use wgpu::Buffer;
use wgpu::util::DeviceExt;

use crate::camera::camera_controller::CameraController;
use crate::camera::main_camera::{Camera, CameraUniform};

pub struct CameraState {
    pub(crate) object: Camera,
    pub(crate) uniform: CameraUniform,
    pub(crate) buffer: Buffer,
    pub(crate) controller: CameraController,
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

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let controller = CameraController::new(0.2);

        Self {
            controller,
            buffer,
            uniform,
            object,
        }
    }

    pub(crate) fn update(&mut self, queue: &wgpu::Queue) {
        self.controller.update_camera(&mut self.object);
        self.uniform.update(&self.object);

        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.uniform]));
    }
}