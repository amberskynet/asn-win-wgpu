use std::sync::Arc;

use asn_gui_core::TAsnSurface;

impl TAsnSurface for super::RenderManager {
    type AsnWindow = asn_winit::winit::window::Window;

    fn init(&mut self, w: Arc<Self::AsnWindow>) -> Result<(), Box<dyn std::error::Error>> {
        let _ = w;
        Ok(())
    }
}
