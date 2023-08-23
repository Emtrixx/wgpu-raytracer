pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
}

impl Camera {
    pub fn build_transform_matrix(&self) -> cgmath::Matrix4<f32> {
        // println!("Camera: {:?}", cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up));
        // let transform = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up) * cgmath::Matrix4::from_translation(cgmath::Vector3::from([self.eye.x, self.eye.y, self.eye.z]));
        // let transform = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let transform = cgmath::Matrix4::look_at_lh(self.eye, self.target, self.up);
        // transform = transform.invert().unwrap();
        return transform;
    }
}

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    // We can't use cgmath with bytemuck directly so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    pub rotation_matrix: [[f32; 4]; 4],
    pub eye: [f32; 3],
    // pub view_params: [f32; 3],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            rotation_matrix: cgmath::Matrix4::identity().into(),
            eye: [0.0, 0.0, 0.0],
        }
    }

    pub fn update(&mut self, camera: &Camera) {
        self.rotation_matrix = camera.build_transform_matrix().into();
        self.eye = camera.eye.into();
    }
}
