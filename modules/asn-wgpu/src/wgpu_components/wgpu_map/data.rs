use super::vertex::Vertex;

pub const LOG_MODULE_NAME: &str = "wgpu_map";

const DELTA_POINT: f32 = 1.0;

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-DELTA_POINT, -DELTA_POINT, 0.0],
        tex_coords: [0.0, 0.0],
    }, // Top-left
    Vertex {
        position: [DELTA_POINT, -DELTA_POINT, 0.0],
        tex_coords: [1.0, 0.0],
    }, // Top-right
    Vertex {
        position: [DELTA_POINT, DELTA_POINT, 0.0],
        tex_coords: [1.0, 1.0],
    }, // Bottom-right
    Vertex {
        position: [-DELTA_POINT, DELTA_POINT, 0.0],
        tex_coords: [0.0, 1.0],
    }, // Bottom-left
];

pub const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

pub const BLUE_PIXEL: &[u8] = &[15, 0, 0xFF, 1];
