@group(0) @binding(0) var color_buffer: texture_storage_2d<rgba8unorm, write>;

@compute @workgroup_size(1,1,1)
fn main(@builtin(global_invocation_id) GlobalInvocationId: vec3<u32>) {

    let screen_pos: vec2<i32> = vec2<i32>(i32(GlobalInvocationId.x), i32(GlobalInvocationId.y));

    let pixel_color: vec3<f32> = vec3<f32>(0.0,1.0,0.0);

    textureStore(color_buffer, screen_pos, vec4<f32>(pixel_color, 1.0));  
}
