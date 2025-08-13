mod render_manager;
pub mod state;
pub mod state_error;
mod wgpu_context;

use render_manager::RenderManager;

mod data;
pub mod rgba_handler;
mod texture;
pub mod wgpu_components;
mod wgpu_utils;

pub use state::State;
pub use state_error::StateError;

pub use rgba_handler::RgbaHandler;
pub use wgpu_components::wgpu_map::WgpuMap;
pub use wgpu_components::wgpu_mesh_color::WgpuQuad;
pub use wgpu_components::wgpu_mesh_textured::WgpuQuadTextured;

pub type ManagerWindow = asn_winit::winit::window::Window;

pub fn get_manager() -> RenderManager {
    RenderManager::new()
}
