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
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

const DELTA_POINT: f32 = 1.0;

const POS_0_0: [f32; 3] = [-DELTA_POINT, -DELTA_POINT, 0.0];
const POS_1_0: [f32; 3] = [DELTA_POINT, -DELTA_POINT, 0.0];
const POS_0_1: [f32; 3] = [-DELTA_POINT, DELTA_POINT, 0.0];
const POS_1_1: [f32; 3] = [DELTA_POINT, DELTA_POINT, 0.0];

const TEX_0_0: [f32; 2] = [0.0, 0.0];
const TEX_1_0: [f32; 2] = [1.0, 0.0];
const TEX_0_1: [f32; 2] = [0.0, 1.0];
const TEX_1_1: [f32; 2] = [1.0, 1.0];

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: POS_0_0,
        tex_coords: TEX_0_0,
    }, // Top-left
    Vertex {
        position: POS_1_0,
        tex_coords: TEX_1_0,
    }, // Top-right
    Vertex {
        position: POS_1_1,
        tex_coords: TEX_1_1,
    }, // Bottom-right
    Vertex {
        position: POS_0_1,
        tex_coords: TEX_0_1,
    }, // Bottom-left
];

pub const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];
// pub const INDICES: &[u16] = &[0, 1, 2];

#[allow(unused)]
pub const BLUE_PIXEL: &[u8] = &[0, 0, 0xFF, 1];
