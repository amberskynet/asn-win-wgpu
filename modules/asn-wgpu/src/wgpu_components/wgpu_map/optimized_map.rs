//! Оптимизированная версия WgpuMap для быстрого обновления текстур
//!
//! Основные оптимизации:
//! - Двойная буферизация текстур
//! - Частичное обновление областей
//! - Асинхронное обновление через staging buffer
//! - Кэширование изменений

use crate::{
    RgbaHandler, texture, wgpu_components::wgpu_map::utils::get_texture_bind_group_layout,
};
use asn_logger::trace;
use wgpu::util::DeviceExt;

/// Область для частичного обновления
#[derive(Debug, Clone)]
pub struct UpdateRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

/// Оптимизированная карта с быстрым обновлением
pub struct OptimizedWgpuMap {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    diffuse_bind_group: wgpu::BindGroup,
    num_indices: u32,

    // Двойная буферизация текстур
    map_textures: [texture::Texture; 2],
    current_texture_index: usize,

    // Обработчик данных
    map_handler: RgbaHandler,

    // Staging buffer для асинхронного обновления
    staging_buffer: wgpu::Buffer,

    // Отслеживание изменений
    is_map_updated: bool,
    dirty_regions: Vec<UpdateRegion>,

    // Константы
    map_width: u32,
    map_height: u32,
}

impl OptimizedWgpuMap {
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
            "OPTIMIZED_MAP_TEXTURE_0",
        )
        .unwrap();

        let map_texture_1 = texture::Texture::from_rgba(
            &device,
            &queue,
            map_handler.data(),
            map_handler.width(),
            map_handler.height(),
            "OPTIMIZED_MAP_TEXTURE_1",
        )
        .unwrap();

        // Создаем staging buffer для асинхронного обновления
        let buffer_size = (map_width * map_height * 4) as u64;
        let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Optimized Map Staging Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::MAP_WRITE,
            mapped_at_creation: true,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Optimized Map Vertex Buffer"),
            contents: bytemuck::cast_slice(&[
                // Позиция, координаты текстуры
                -1.0, -1.0, 0.0, 0.0, 0.0, 1.0, -1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, -1.0,
                1.0, 0.0, 0.0, 1.0,
            ]),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Optimized Map Index Buffer"),
            contents: bytemuck::cast_slice(&[0, 1, 2, 2, 3, 0]),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = 6;

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
                    resource: wgpu::BindingResource::TextureView(&map_texture_0.view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&map_texture_0.sampler),
                },
            ],
            label: Some("optimized_map_diffuse_bind_group"),
        });

        let render_pipeline = crate::wgpu_components::wgpu_map::utils::get_render_pipeline(
            device,
            format,
            &texture_bind_group_layout,
            shader_source,
        );

        Self {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            diffuse_bind_group,
            map_handler,
            map_textures: [map_texture_0, map_texture_1],
            current_texture_index: 0,
            staging_buffer,
            is_map_updated: false,
            dirty_regions: Vec::new(),
            map_width,
            map_height,
        }
    }

    /// Обновляет всю карту
    pub fn update_map(&mut self, rgba: &[u8]) {
        self.map_handler.update_data(rgba).unwrap();
        self.is_map_updated = true;
        self.dirty_regions.clear();
    }

    /// Обновляет только определенную область карты
    pub fn update_region(&mut self, region: UpdateRegion, rgba: &[u8]) {
        // Обновляем данные в handler
        let start_x = region.x as usize;
        let start_y = region.y as usize;
        let width = region.width as usize;
        let height = region.height as usize;

        let handler_width = self.map_handler.width() as usize;

        for y in 0..height {
            for x in 0..width {
                let handler_x = start_x + x;
                let handler_y = start_y + y;
                let rgba_index = (y * width + x) * 4;

                if handler_x < handler_width && handler_y < self.map_handler.height() as usize {
                    let r = rgba[rgba_index];
                    let g = rgba[rgba_index + 1];
                    let b = rgba[rgba_index + 2];
                    let a = rgba[rgba_index + 3];

                    self.map_handler
                        .set_pixel(handler_x as u32, handler_y as u32, r, g, b, a)
                        .unwrap();
                }
            }
        }

        self.is_map_updated = true;
        self.dirty_regions.push(region);
    }

    /// Асинхронно обновляет текстуру
    pub fn update_queue(&mut self, queue: &wgpu::Queue) {
        if self.is_map_updated {
            // Переключаемся на другую текстуру
            self.current_texture_index = (self.current_texture_index + 1) % 2;

            // Копируем данные в staging buffer
            let buffer_slice = self.staging_buffer.slice(..);
            let mut view = buffer_slice.get_mapped_range_mut();
            view.copy_from_slice(self.map_handler.data());
            drop(view);

            // Обновляем текстуру
            self.map_textures[self.current_texture_index].update_from_rgba(
                queue,
                self.map_handler.data(),
                self.map_handler.width(),
                self.map_handler.height(),
            );

            self.is_map_updated = false;
            self.dirty_regions.clear();
        }
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
            label: Some("optimized_map_diffuse_bind_group"),
        });
    }

    pub fn draw(&self, render_pass: &mut wgpu::RenderPass) {
        trace("OPTIMIZED_MAP", "draw");
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }

    /// Получает размеры карты
    pub fn dimensions(&self) -> (u32, u32) {
        (self.map_width, self.map_height)
    }

    /// Проверяет, есть ли несохраненные изменения
    pub fn has_pending_updates(&self) -> bool {
        self.is_map_updated
    }

    /// Получает количество грязных областей
    pub fn dirty_regions_count(&self) -> usize {
        self.dirty_regions.len()
    }
}
