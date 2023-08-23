@group(0) @binding(0) var color_buffer: texture_2d<f32>;
@group(0) @binding(1) var screen_sampler: sampler;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) texCoords: vec2<f32>,

};

struct VertexOutput{
    @builtin(position) Position: vec4<f32>,
    @location(0) TexCoord: vec2<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {

    // let positions = array<vec2<f32>, 6>(
    //     vec2<f32>(1.0, 1.0),
    //     vec2<f32>(1.0, -1.0),
    //     vec2<f32>(-1.0, -1.0),
    //     vec2<f32>(1.0, 1.0),
    //     vec2<f32>(-1.0, -1.0),
    //     vec2<f32>(-1.0, 1.0)
    // );

    // let texCoords = array<vec2<f32>, 6>(
    //     vec2<f32>(1.0, 0.0),
    //     vec2<f32>(1.0, 1.0),
    //     vec2<f32>(0.0, 1.0),
    //     vec2<f32>(1.0, 0.0),
    //     vec2<f32>(0.0, 1.0),
    //     vec2<f32>(0.0, 0.0)
    // );

    var output: VertexOutput;
    output.Position = vec4<f32>(in.position, 0.0, 1.0);
    output.TexCoord = in.texCoords;
    return output;
}

@fragment
fn fs_main(@location(0) TexCoord: vec2<f32>) -> @location(0) vec4<f32> {
    return textureSample(color_buffer, screen_sampler, TexCoord);
}