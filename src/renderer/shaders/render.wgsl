// Vertex shader for fullscreen quad
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(@location(0) position: vec2<f32>) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4<f32>(position, 0.0, 1.0);
    // Convert from [-1, 1] to [0, 1] for texture coordinates
    output.tex_coords = position * 0.5 + 0.5;
    // Flip Y coordinate (texture coordinate origin is top-left)
    output.tex_coords.y = 1.0 - output.tex_coords.y;
    return output;
}

// Fragment shader
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, input.tex_coords);
}
