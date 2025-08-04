// Оптимизированный шейдер для быстрого обновления карты

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader с оптимизациями

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;
@group(0) @binding(2)
var t_map: texture_2d<f32>;
@group(0) @binding(3)
var s_map: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Оптимизированное смешивание текстур
    let base_color = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    let map_color = textureSample(t_map, s_map, in.tex_coords);
    
    // Используем более эффективное смешивание
    return mix(base_color, map_color, map_color.a);
} 