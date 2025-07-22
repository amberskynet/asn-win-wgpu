use super::vertex::Vertex;

pub const LOG_MODULE_NAME: &str = "wgpu_map";

pub const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.5,  0.5, 0.0], tex_coords: [0.0, 0.0] }, // Top-left
    Vertex { position: [ 0.5,  0.5, 0.0], tex_coords: [1.0, 0.0] }, // Top-right
    Vertex { position: [ 0.5, -0.5, 0.0], tex_coords: [1.0, 1.0] }, // Bottom-right
    Vertex { position: [-0.5, -0.5, 0.0], tex_coords: [0.0, 1.0] }, // Bottom-left
];

pub const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0]; 