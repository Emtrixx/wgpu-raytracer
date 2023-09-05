use cgmath::Angle;

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
        // TODO: rotation and translation at once?
        let transform = cgmath::Matrix4::look_at_lh(self.eye, self.target, self.up);
        // transform = transform.invert().unwrap();
        return transform;
    }

    // pub fn change_fovy(&mut self, fovy: f32) {
    //     self.fovy = fovy;
    // }
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
    pub _padding: u32,
    pub view_params: [f32; 3],
    pub _padding2: u32,
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            rotation_matrix: cgmath::Matrix4::identity().into(),
            eye: [0.0, 0.0, 0.0],
            _padding: 0,
            view_params: [0.0, 0.0, 0.0],
            _padding2: 0,
        }
    }

    pub fn update_view_params(&mut self, camera: &Camera) {
        let angle = cgmath::Deg(camera.fovy * 0.5).tan();
        let plane_height: f32 = 2.0 * angle * camera.znear;
        let plane_width: f32 = plane_height * camera.aspect;

        self.view_params = [plane_width, plane_height, camera.znear];
    }

    pub fn update(&mut self, camera: &Camera) {
        self.rotation_matrix = camera.build_transform_matrix().into();
        self.eye = camera.eye.into();

        // TODO: only on param change (maybe use events?)
        self.update_view_params(camera);
    }
}
