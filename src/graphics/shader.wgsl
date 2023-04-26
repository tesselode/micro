// Vertex shader

struct DrawParamsUniform {
    transform: mat4x4<f32>,
    color: vec4<f32>,
}

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) texture_coords: vec2<f32>,
    @location(2) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) texture_coords: vec2<f32>,
    @location(1) color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> params: DrawParamsUniform;

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.texture_coords = model.texture_coords;
    out.color = model.color * params.color;
    out.clip_position = params.transform * vec4<f32>(model.position, 0.0, 1.0);
    return out;
}

// Fragment shader

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color * textureSample(t_diffuse, s_diffuse, in.texture_coords);
}
