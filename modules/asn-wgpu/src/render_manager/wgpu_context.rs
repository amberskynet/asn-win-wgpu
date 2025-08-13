use std::sync::Arc;

use asn_winit::WinitWindow;

pub struct WgpuContext {
    // pub device: wgpu::Device,
    // pub queue: wgpu::Queue,
    // pub surface_format: wgpu::TextureFormat,
    // pub surface: wgpu::Surface<'static>,
    // pub config: wgpu::SurfaceConfiguration,
    // pub is_surface_configured: bool,
}

impl WgpuContext {
    pub fn new(w: Arc<WinitWindow>) -> Self {
        let _ = w;
        WgpuContext {}
    }
}
