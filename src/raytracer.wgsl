@group(0) @binding(0) var color_buffer: texture_storage_2d<rgba8unorm, write>;

struct Globals {
    @builtin(num_workgroups) num_workgroups: vec3<u32>,
    @builtin(global_invocation_id) globalInvocationId: vec3<u32>,
};

@compute @workgroup_size(1,1,1)
fn main(globals: Globals) {

    let screen_pos: vec2<i32> = vec2<i32>(i32(globals.globalInvocationId.x), i32(globals.globalInvocationId.y));

    // Camera parameters
    let eye: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    let fov: f32 = 60.0;
    let aspect: f32 = f32(globals.num_workgroups.x) / f32(globals.num_workgroups.y);
    let near: f32 = 0.01;

    // Sphere parameters
    let sphere_center: vec3<f32> = vec3<f32>(0.0, 0.0, -5.0);
    let sphere_radius: f32 = 1.0;

    // Calculate ray direction for current pixel
    let plane_height: f32 = 2.0 * tan(fov * 0.5 * 3.14159 / 180.0) * near;
    let plane_width: f32 = plane_height * aspect;
    let pixel_width: f32 = plane_width / f32(globals.num_workgroups.x);
    let pixel_height: f32 = plane_height / f32(globals.num_workgroups.y);
    let pixel_pos: vec2<f32> = vec2<f32>(f32(screen_pos.x) * pixel_width, f32(screen_pos.y) * pixel_height);
    let ray_dir: vec3<f32> = normalize(vec3<f32>(pixel_pos.x, pixel_pos.y, -near));
    let pixel_pos_world: vec3<f32> = eye + ray_dir;

    // Check for intersection with sphere
    let sphere_to_pixel: vec3<f32> = pixel_pos_world - sphere_center;
    let a: f32 = dot(ray_dir, ray_dir);
    let b: f32 = 2.0 * dot(sphere_to_pixel, ray_dir);
    let c: f32 = dot(sphere_to_pixel, sphere_to_pixel) - sphere_radius * sphere_radius;
    let discriminant: f32 = b * b - 4.0 * a * c;
    if (discriminant < 0.0) {
        // No intersection, set pixel color to black
        textureStore(color_buffer, screen_pos, vec4<f32>(0.0, 0.0, 0.0, 1.0));
    } else {
        // Intersection, set pixel color to green
        textureStore(color_buffer, screen_pos, vec4<f32>(0.0, 1.0, 0.0, 1.0));
    }
}
