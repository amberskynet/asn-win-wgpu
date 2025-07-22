use std::sync::Arc;
use std::time::Instant;

use asn_logger::trace;
use winit::window::Window;

use crate::{
    data::{DEFAULT_CLEAR_COLOR, LOG_MODULE_NAME, MIN_WINDOW_SIZE},
    state_error::StateError,
    wgpu_components::{wgpu_map, wgpu_mesh_color, wgpu_mesh_textured},
};

/// GPU state and rendering management
pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    window: Arc<Window>,
    quad: wgpu_mesh_color::WgpuQuad,
    quad_textured: wgpu_mesh_textured::WgpuQuadTextured,
    quad_map: wgpu_map::WgpuMap,
    render_stats: RenderStats,
}

/// Performance monitoring statistics
#[derive(Default)]
struct RenderStats {
    frame_count: u32,
    total_render_time: std::time::Duration,
    last_frame_time: Option<Instant>,
}

/// Render context for split pass rendering
pub struct RenderContext {
    pub output: wgpu::SurfaceTexture,
    pub encoder: wgpu::CommandEncoder,
    pub view: wgpu::TextureView,
    pub frame_start: Instant,
}

impl State {
    /// Creates new GPU state with specified window
    ///
    /// # Arguments
    /// * `window` - Window for rendering
    ///
    /// # Returns
    /// * `Result<Self, StateError>` - New state or error
    pub async fn new(window: Arc<Window>) -> Result<Self, StateError> {
        trace(LOG_MODULE_NAME, "Creating new State");

        let size = window.inner_size();
        trace(LOG_MODULE_NAME, &format!("window size: {size:?}"));

        // Window size validation
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

        // Find suitable adapter
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

        trace(LOG_MODULE_NAME, "State created successfully");

        let shader_source = include_str!("color_triangle.wgsl");
        let quad = wgpu_mesh_color::WgpuQuad::new(&device, surface_format, shader_source);

        let diffuse_bytes = include_bytes!("happy-tree.png");
        let shader_source = include_str!("textured_triangle.wgsl");

        let quad_textured = wgpu_mesh_textured::WgpuQuadTextured::new(
            &device,
            &queue,
            surface_format,
            shader_source,
            diffuse_bytes,
        );

        let shader_source = include_str!("map_shader.wgsl");
        let map_bytes = include_bytes!("tiles.png");
        let quad_map =
            wgpu_map::WgpuMap::new(&device, &queue, surface_format, shader_source, map_bytes);

        Ok(Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            window,
            quad,
            quad_textured,
            quad_map,
            render_stats: RenderStats::default(),
        })
    }

    /// Resizes the rendering surface
    ///
    /// # Arguments
    /// * `width` - New width
    /// * `height` - New height
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), StateError> {
        trace(LOG_MODULE_NAME, &format!("resize {width} {height}"));

        // Size validation
        if width < MIN_WINDOW_SIZE || height < MIN_WINDOW_SIZE {
            return Err(StateError::InvalidWindowSize { width, height });
        }

        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
        self.is_surface_configured = true;

        Ok(())
    }

    /// Restores state after context loss
    pub fn restore(&mut self) -> Result<(), StateError> {
        let size = self.window.inner_size();
        self.resize(size.width, size.height)
    }

    /// Starts render pass, returns RenderContext
    pub fn draw_start(&mut self) -> Result<RenderContext, StateError> {
        self.window.request_redraw();
        if !self.is_surface_configured {
            return Err(StateError::TextureError(
                "Surface not configured".to_string(),
            ));
        }
        let output = self
            .surface
            .get_current_texture()
            .map_err(|e| StateError::TextureError(e.to_string()))?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let frame_start = Instant::now();

        Ok(RenderContext {
            output,
            encoder,
            view,
            frame_start,
        })
    }

    /// Ends render pass, submits commands and presents output
    pub fn draw_end(&mut self, ctx: RenderContext) -> Result<(), StateError> {
        let frame_duration = ctx.frame_start.elapsed();

        // Update render statistics
        self.render_stats.frame_count += 1;
        self.render_stats.total_render_time += frame_duration;

        // Log performance every 60 frames
        if self.render_stats.frame_count % 60 == 0 {
            let avg_frame_time =
                self.render_stats.total_render_time / self.render_stats.frame_count;
            let fps = 1.0 / avg_frame_time.as_secs_f64();
            trace(
                LOG_MODULE_NAME,
                &format!("Avg FPS: {:.1}, Frame time: {:?}", fps, avg_frame_time),
            );
        }

        self.queue.submit(std::iter::once(ctx.encoder.finish()));
        ctx.output.present();
        Ok(())
    }

    /// Performs rendering with the provided context
    pub fn draw(&mut self, ctx: &mut RenderContext) -> Result<(), StateError> {
        let mut render_pass = ctx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &ctx.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(DEFAULT_CLEAR_COLOR),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        self.quad_map.draw(&mut render_pass);
        // self.quad.draw(&mut render_pass);
        // self.quad_textured.draw(&mut render_pass);

        Ok(())
    }

    /// Returns window size
    pub fn window_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.window.inner_size()
    }

    /// Checks if surface is configured
    pub fn is_configured(&self) -> bool {
        self.is_surface_configured
    }

    /// Returns current frame count
    pub fn frame_count(&self) -> u64 {
        self.render_stats.frame_count as u64
    }

    /// Returns average frame time
    pub fn average_frame_time(&self) -> Option<std::time::Duration> {
        if self.render_stats.frame_count > 0 {
            Some(self.render_stats.total_render_time / self.render_stats.frame_count)
        } else {
            None
        }
    }

    /// Returns current FPS
    pub fn fps(&self) -> Option<f64> {
        self.average_frame_time()
            .map(|duration| 1.0 / duration.as_secs_f64())
    }
}
