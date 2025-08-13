use std::sync::Arc;

use asn_logger::*;
use asn_winit::WinitWindow;

use crate::{
    StateError,
    data::{LOG_MODULE_NAME, MIN_WINDOW_SIZE},
};

pub struct WgpuContext {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_format: wgpu::TextureFormat,
    pub surface: wgpu::Surface<'static>,
    pub config: wgpu::SurfaceConfiguration,
}

impl WgpuContext {
    pub async fn new(window: Arc<WinitWindow>) -> Result<Self, StateError> {
        let size = window.inner_size();
        trace(LOG_MODULE_NAME, &format!("window size: {size:?}"));

        if size.width < MIN_WINDOW_SIZE || size.height < MIN_WINDOW_SIZE {
            return Err(StateError::InvalidWindowSize {
                width: size.width,
                height: size.height,
            });
        }

        let backend_features = wgpu::Instance::enabled_backend_features();
        trace(
            LOG_MODULE_NAME,
            &format!("backend_features: {backend_features:?}"),
        );

        // Create GPU instance
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        // Create surface
        let surface = instance
            .create_surface(window.clone())
            .map_err(|e| StateError::SurfaceCreation(e.to_string()))?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .map_err(|_| StateError::NoAdapter)?;

        // Create device and queue
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("ASN WGPU Device"),
                required_features: wgpu::Features::empty(),
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await
            .map_err(|e| StateError::DeviceCreation(e.to_string()))?;

        // Configure surface
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let w = WgpuContext {
            device,
            queue,
            surface_format,
            surface,
            config,
        };

        Ok(w)
    }
}
