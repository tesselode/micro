// Vertex shader

struct DrawParams {
    transform: mat4x4<f32>,
    local_transform: mat4x4<f32>,
    color: vec4<f32>,
}
@group(0) @binding(0)
var<uniform> draw_params: DrawParams;

struct Instance {
    translation: vec2<f32>,
}
@group(2) @binding(0)
var<storage> instances: array<Instance>;

struct VertexInput {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec2<f32>,
    @location(1) texture_coords: vec2<f32>,
    @location(2) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) texture_coords: vec2<f32>,
    @location(1) color: vec4<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var position = model.position + instances[model.instance_index].translation;
    var out: VertexOutput;
    out.clip_position = draw_params.transform * vec4<f32>(position, 0.0, 1.0);
    out.texture_coords = model.texture_coords;
    out.color = model.color * draw_params.color;
    return out;
}

// Fragment shader

@group(0) @binding(1)
var texture_view: texture_2d<f32>;
@group(0) @binding(2)
var texture_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color * textureSample(texture_view, texture_sampler, in.texture_coords);
}
