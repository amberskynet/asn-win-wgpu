extern crate asn_winit;

pub mod render_manager;
mod state_error;

// use std::sync::{Arc, Mutex};
// use asn_gui_core::TAsnGuiHandler;
// use render_manager::RenderManager;

mod data;

pub use state_error::StateError;

pub type GraphContext = render_manager::WgpuContext;
pub type FrameContext = render_manager::WgpuFrameContext;

// pub fn get_manager<
//     H: TAsnGuiHandler<
//             GraphContext = render_manager::WgpuContext,
//             FrameContext = render_manager::WgpuFrameContext,
//         >,
// >(
//     h: Arc<Mutex<H>>,
// ) -> impl asn_winit::RenderManager {
//     RenderManager::new(h)
// }
