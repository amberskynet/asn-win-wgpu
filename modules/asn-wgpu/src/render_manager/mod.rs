mod asn_render_manager_impl;
mod frame_context;
mod wgpu_context;

use std::sync::{Arc, Mutex};

pub use asn_render_manager_impl::RenderManagerError;
pub use frame_context::WgpuFrameContext;
pub use wgpu_context::WgpuGraphContext;

use crate::WgpuGuiHandler;

/// Менеджер рендеринга, отвечающий за управление графическим контекстом и отрисовку
pub struct RenderManager<H>
where
    H: WgpuGuiHandler,
{
    /// Графический контекст (может быть None, если менеджер не инициализирован)
    s: Option<wgpu_context::WgpuGraphContext>,
    /// Обработчик GUI
    h: Arc<Mutex<H>>,
    /// Статистика рендеринга
    render_stats: RenderStats,
}

/// Статистика рендеринга для мониторинга производительности
struct RenderStats {
    /// Количество отрендеренных кадров
    frame_count: u64,
    /// Общее время рендеринга
    total_render_time: web_time::Duration,
}

impl<H> RenderManager<H>
where
    H: WgpuGuiHandler,
{
    pub fn new(h: Arc<Mutex<H>>) -> Self {
        RenderManager {
            s: None,
            h: h.clone(),
            render_stats: RenderStats {
                frame_count: 0,
                total_render_time: web_time::Duration::new(0, 0),
            },
        }
    }
}
