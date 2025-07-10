use std::sync::Arc;

use asn_logger::trace;
use winit::window::Window;

use crate::{
    data::{DEFAULT_CLEAR_COLOR, LOG_MODULE_NAME, MIN_WINDOW_SIZE},
    state_error::StateError,
    wgpu_components::wgpu_quad,
    wgpu_utils::get_render_pipeline,
};

/// Состояние GPU и рендеринга
pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    window: Arc<Window>,
    quad: wgpu_quad::WgpuQuad,
}

/// Контекст рендера для split pass
pub struct RenderContext {
    pub output: wgpu::SurfaceTexture,
    pub encoder: wgpu::CommandEncoder,
    pub view: wgpu::TextureView,
}

impl State {
    /// Создает новое состояние GPU с указанным окном
    ///
    /// # Arguments
    /// * `window` - Окно для рендеринга
    ///
    /// # Returns
    /// * `Result<Self, StateError>` - Новое состояние или ошибка
    pub async fn new(window: Arc<Window>) -> Result<Self, StateError> {
        trace(LOG_MODULE_NAME, "Creating new State");

        let size = window.inner_size();
        trace(LOG_MODULE_NAME, &format!("window size: {size:?}"));

        // Валидация размера окна
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

        // Создание экземпляра GPU
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        // Создание поверхности
        let surface = instance
            .create_surface(window.clone())
            .map_err(|e| StateError::SurfaceCreation(e.to_string()))?;

        // Поиск подходящего адаптера
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .map_err(|_| StateError::NoAdapter)?;

        // Создание устройства и очереди
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

        // Настройка поверхности
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

        let render_pipeline = get_render_pipeline(&device, surface_format);

        trace(LOG_MODULE_NAME, "State created successfully");

        let quad = wgpu_quad::WgpuQuad::new(render_pipeline);
        
        Ok(Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            window,
            quad,
        })
    }

    /// Изменяет размер поверхности рендеринга
    ///
    /// # Arguments
    /// * `width` - Новая ширина
    /// * `height` - Новая высота
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), StateError> {
        trace(LOG_MODULE_NAME, &format!("resize {width} {height}"));

        // Валидация размеров
        if width < MIN_WINDOW_SIZE || height < MIN_WINDOW_SIZE {
            return Err(StateError::InvalidWindowSize { width, height });
        }

        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
        self.is_surface_configured = true;

        Ok(())
    }

    /// Восстанавливает состояние после потери контекста
    pub fn restore(&mut self) -> Result<(), StateError> {
        let size = self.window.inner_size();
        self.resize(size.width, size.height)
    }

    /// Начинает рендер-проход, возвращает RenderContext
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
        Ok(RenderContext {
            output,
            encoder,
            view,
        })
    }

    /// Завершает рендер-проход, сабмитит команды и презентует output
    pub fn draw_end(&mut self, ctx: RenderContext) -> Result<(), StateError> {
        self.queue.submit(std::iter::once(ctx.encoder.finish()));
        ctx.output.present();
        Ok(())
    }

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
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        self.quad.draw(&mut render_pass);
        Ok(())
    }

    /// Возвращает размер окна
    pub fn window_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.window.inner_size()
    }

    /// Проверяет, настроена ли поверхность
    pub fn is_configured(&self) -> bool {
        self.is_surface_configured
    }
}
