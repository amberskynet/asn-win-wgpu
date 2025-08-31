mod vertex;

use asn_wgpu::wgpu;
pub use vertex::Vertex;
pub mod rgba_handler;
pub mod texture;
pub mod utils;

pub const SHADER_SOURCE: &str = include_str!("map_shader.wgsl");
#[allow(dead_code)]
pub const LOG_MODULE_NAME: &str = "wgpu_map";

pub const DEFAULT_CLEAR_COLOR: wgpu::Color = wgpu::Color {
    r: 0.1,
    g: 0.2,
    b: 0.3,
    a: 1.0,
};

const DELTA_POINT: f32 = 1.0;

const TEX_0_0: [f32; 2] = [0.0, 0.0];
const TEX_1_0: [f32; 2] = [1.0, 0.0];
const TEX_0_1: [f32; 2] = [0.0, 1.0];
const TEX_1_1: [f32; 2] = [1.0, 1.0];

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-DELTA_POINT, -DELTA_POINT, 0.0],
        tex_coords: TEX_1_1,
    }, // Top-left
    Vertex {
        position: [DELTA_POINT, -DELTA_POINT, 0.0],
        tex_coords: TEX_0_1,
    }, // Top-right
    Vertex {
        position: [DELTA_POINT, DELTA_POINT, 0.0],
        tex_coords: TEX_0_0,
    }, // Bottom-right
    Vertex {
        position: [-DELTA_POINT, DELTA_POINT, 0.0],
        tex_coords: TEX_1_0,
    }, // Bottom-left
];

pub const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

#[allow(unused)]
pub const BLUE_PIXEL: &[u8] = &[0, 0, 0xFF, 1];
