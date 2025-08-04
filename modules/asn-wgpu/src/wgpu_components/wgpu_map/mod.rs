//! # WGPU Map Module
//!
//! Модуль `wgpu_map` предоставляет функциональность для работы с картами и текстурами в WGPU.
//!
//! ## Компоненты
//!
//! ### RgbaHandler
//!
//! `RgbaHandler` - это класс для инициализации и обработки RGBA-массивов. Он предоставляет удобный интерфейс для работы с пиксельными данными изображений.
//!
//! #### Основные возможности
//!
//! - **Создание RGBA-массивов** с указанными размерами
//! - **Манипуляция пикселями** - установка и получение цветов отдельных пикселей
//! - **Заполнение областей** - заполнение всего изображения или областей указанным цветом
//! - **Создание градиентов** - линейные градиенты между двумя цветами
//! - **Создание текстур** - интеграция с WGPU для создания текстур из RGBA-данных
//! - **Валидация данных** - проверка корректности размеров и координат
//!
//! ### WgpuMap
//!
//! `WgpuMap` - основной класс для рендеринга карт в WGPU. Использует `RgbaHandler` для управления текстурами карт.
//!
//! ## Примеры использования
//!
//! ```rust
//! use asn_wgpu::RgbaHandler;
//!
//! // Создание нового RGBA-обработчика
//! let mut handler = RgbaHandler::new(256, 256);
//!
//! // Заполнение синим цветом
//! handler.fill(0, 0, 255, 255);
//!
//! // Установка красного пикселя
//! handler.set_pixel(100, 100, 255, 0, 0, 255).unwrap();
//!
//! // Создание градиента
//! handler.create_gradient((255, 0, 0, 255), (0, 0, 255, 255));
//! ```
//!
//! ## Структура файлов
//!
//! ```
//! wgpu_map/
//! ├── mod.rs          # Основной модуль с экспортами
//! ├── data.rs         # Константы и данные
//! ├── rgba_handler.rs # Класс RgbaHandler
//! ├── utils.rs        # Утилиты для WGPU
//! ├── vertex.rs       # Структуры вершин
//! └── README.md       # Документация
//! ```

mod data;
pub mod optimized_map;
mod utils;
mod vertex;

use crate::{
    RgbaHandler, state_error::StateError, texture,
    wgpu_components::wgpu_map::utils::get_texture_bind_group_layout,
};

// Реэкспортируем оптимизированную карту
use asn_logger::trace;
use data::{INDICES, LOG_MODULE_NAME, VERTICES};
pub use optimized_map::{OptimizedWgpuMap, UpdateRegion};
use utils::get_render_pipeline;
use wgpu::util::DeviceExt;

pub struct WgpuMap {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    diffuse_bind_group: wgpu::BindGroup,
    num_indices: u32,
    map_handler: RgbaHandler,
    map_textures: [texture::Texture; 2], // Двойная буферизация
    current_texture_index: usize,
    is_map_updated: bool,
    update_buffer: wgpu::Buffer, // Буфер для асинхронного обновления
}

impl WgpuMap {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        shader_source: &str,
        map_tiles_bytes: &[u8],
        map_width: u32,
        map_height: u32,
    ) -> Self {
        let diffuse_texture =
            texture::Texture::from_bytes(&device, &queue, map_tiles_bytes, "map-texture.png")
                .unwrap();

        let mut map_handler = RgbaHandler::new(map_width, map_height);
        map_handler.fill_random();

        // Создаем две текстуры для двойной буферизации
        let map_texture_0 = texture::Texture::from_rgba(
            &device,
            &queue,
            map_handler.data(),
            map_handler.width(),
            map_handler.height(),
            "MAP_TEXTURE_0",
        )
        .unwrap();

        let map_texture_1 = texture::Texture::from_rgba(
            &device,
            &queue,
            map_handler.data(),
            map_handler.width(),
            map_handler.height(),
            "MAP_TEXTURE_1",
        )
        .unwrap();

        // Создаем буфер для асинхронного обновления
        let buffer_size = (map_width * map_height * 4) as u64;
        let update_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Map Update Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::MAP_WRITE,
            mapped_at_creation: true,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Map Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Map Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = INDICES.len() as u32;

        let texture_bind_group_layout = get_texture_bind_group_layout(&device);

        // Создаем bind group с первой текстурой
        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&map_texture_0.view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&map_texture_0.sampler),
                },
            ],
            label: Some("map_diffuse_bind_group"),
        });

        let render_pipeline =
            get_render_pipeline(device, format, &texture_bind_group_layout, shader_source);

        Self {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            diffuse_bind_group,
            map_handler,
            map_textures: [map_texture_0, map_texture_1],
            current_texture_index: 0,
            is_map_updated: false,
            update_buffer,
        }
    }

    pub fn update_map(&mut self, rgba: &[u8]) {
        self.map_handler.update_data(rgba).unwrap();
        self.is_map_updated = true;
    }

    pub fn update_queue(&mut self, queue: &wgpu::Queue) {
        if self.is_map_updated {
            // Переключаемся на другую текстуру
            self.current_texture_index = (self.current_texture_index + 1) % 2;

            // Обновляем текстуру напрямую
            self.map_textures[self.current_texture_index].update_from_rgba(
                queue,
                self.map_handler.data(),
                self.map_handler.width(),
                self.map_handler.height(),
            );

            self.is_map_updated = false;
        }
    }

    pub fn draw(&self, render_pass: &mut wgpu::RenderPass) {
        trace(LOG_MODULE_NAME, format!("draw").as_str());
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }

    /// Обновляет bind group с текущей активной текстурой
    pub fn update_bind_group(&mut self, device: &wgpu::Device) {
        let texture_bind_group_layout = get_texture_bind_group_layout(device);
        self.diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.map_textures[0].view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.map_textures[0].sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(
                        &self.map_textures[self.current_texture_index].view,
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(
                        &self.map_textures[self.current_texture_index].sampler,
                    ),
                },
            ],
            label: Some("map_diffuse_bind_group"),
        });
    }

    /// Перезагружает шейдер карты с диска
    pub fn reload_shader(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        texture_bytes: &[u8],
    ) -> Result<(), StateError> {
        use std::fs;
        let shader_source = fs::read_to_string("modules/asn-wgpu/src/map_shader.wgsl")
            .map_err(|e| StateError::TextureError(format!("Failed to reload shader: {e}")))?;

        // Создаем новый экземпляр WgpuMap с обновленным шейдером
        let new_map = Self::new(
            device,
            queue,
            format,
            &shader_source,
            texture_bytes,
            self.map_handler.width(),
            self.map_handler.height(),
        );

        // Обновляем текущий экземпляр
        *self = new_map;

        Ok(())
    }
}
