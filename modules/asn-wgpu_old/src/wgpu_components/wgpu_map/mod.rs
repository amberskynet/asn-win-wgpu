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
mod impl_gui_element;
mod impl_gui_map;
mod utils;
mod vertex;

use crate::{
    RgbaHandler, texture, wgpu_components::wgpu_map::utils::get_texture_bind_group_layout,
};

use asn_logger::m_trace;
use data::{INDICES, LOG_MODULE_NAME, SHADER_SOURCE, VERTICES};
use utils::get_render_pipeline;
use wgpu::util::DeviceExt;

pub struct WgpuMap {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    diffuse_bind_group: wgpu::BindGroup,
    num_indices: u32,
    map_handler: RgbaHandler,
    map_texture: texture::Texture,
    is_map_updated: bool,
}

impl WgpuMap {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        map_tiles_bytes: &[u8],
        map_width: u32,
        map_height: u32,
    ) -> Self {
        let diffuse_texture =
            texture::Texture::from_bytes(device, queue, map_tiles_bytes, "map-texture.png")
                .unwrap();

        let mut map_handler = RgbaHandler::new(map_width, map_height);
        map_handler.fill_random();

        let map_texture = texture::Texture::from_rgba(
            device,
            queue,
            map_handler.data(),
            map_handler.width(),
            map_handler.height(),
            "MAP_TEXTURE_0",
        )
        .unwrap();

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
                    resource: wgpu::BindingResource::TextureView(&map_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&map_texture.sampler),
                },
            ],
            label: Some("map_diffuse_bind_group"),
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Map Shader"),
            source: wgpu::ShaderSource::Wgsl(SHADER_SOURCE.into()),
        });

        let render_pipeline =
            get_render_pipeline(device, format, &texture_bind_group_layout, shader);

        Self {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            diffuse_bind_group,
            map_handler,
            map_texture,
            is_map_updated: false,
        }
    }

    pub fn update_map(&mut self, rgba: &[u8]) {
        self.map_handler.update_data(rgba).unwrap();
        self.is_map_updated = true;
    }

    pub fn update_queue(&mut self, queue: &wgpu::Queue) {
        if self.is_map_updated {
            // Обновляем текстуру напрямую
            self.map_texture.update_from_rgba(
                queue,
                self.map_handler.data(),
                self.map_handler.width(),
                self.map_handler.height(),
            );

            self.is_map_updated = false;
        }
    }

    pub fn draw(&self, render_pass: &mut wgpu::RenderPass) {
        m_trace!("draw");
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}
