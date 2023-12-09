use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GlobalUniform {
    pub timestamp: u32,
}

impl GlobalUniform {
    pub fn new() -> Self {
        Self {
            timestamp: 0,
        }
    }
}

pub struct GlobalState {
    pub buffer: wgpu::Buffer,
    pub uniform: GlobalUniform,
}

impl GlobalState {
    pub fn new(device: &wgpu::Device) -> Self {
        let uniform = GlobalUniform::new();

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Global Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            buffer,
            uniform,
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue) {
        self.uniform.timestamp += 1;

        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.uniform]));
    }
}