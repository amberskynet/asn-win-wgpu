use asn_core::cgmath::{Matrix4, SquareMatrix};
use asn_gui_core::TAsnGuiElement;
use asn_wgpu::wgpu::util::DeviceExt;
use asn_wgpu::{WgpuFrameContext, WgpuGraphContext, wgpu};

use crate::data::rgba_handler::RgbaHandler;
use crate::data::texture::{AsnTextureFormat, WgpuTexture};
use crate::data::utils::{get_render_pipeline, get_texture_bind_group_layout};
use crate::data::{DEFAULT_CLEAR_COLOR, INDICES, SHADER_SOURCE, VERTICES};

/// Оптимизированная версия WgpuMap с двойной буферизацией для плавного рендеринга
pub struct OptimizedWgpuMap {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    diffuse_bind_groups: [wgpu::BindGroup; 2], // Две bind группы для двойной буферизации
    num_indices: u32,
    map_handler: RgbaHandler,
    map_textures: [WgpuTexture; 2], // Две текстуры карты
    current_texture_index: usize,   // Индекс текущей текстуры для рендеринга
    /// Texture containing tile sprites - kept alive for bind group
    tiles_texture: WgpuTexture,
    /// Uniform buffer with tile and map information - kept alive for bind group
    tiles_info_buffer: wgpu::Buffer,
    mvp_matrix_buffer: wgpu::Buffer, // Uniform-буфер для MVP-матрицы
    update_pending: bool,            // Флаг, что обновление ожидает
    staging_buffer: Option<wgpu::Buffer>, // Staging buffer для асинхронных обновлений
}

mod data;

pub struct WgpuMap {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    diffuse_bind_group: wgpu::BindGroup,
    num_indices: u32,
    map_handler: RgbaHandler,
    map_texture: WgpuTexture,
    /// Texture containing tile sprites - kept alive for bind group
    tiles_texture: WgpuTexture,
    /// Uniform buffer with tile and map information - kept alive for bind group
    tiles_info_buffer: wgpu::Buffer,
    mvp_matrix_buffer: wgpu::Buffer, // Uniform-буфер для MVP-матрицы
    is_map_updated: bool,
}

impl WgpuMap {
    pub fn update_map(&mut self, map_indices: &[u32]) -> Result<(), Box<dyn std::error::Error>> {
        // Получаем ширину карты из текущего обработчика
        let map_width = self.map_handler.width();

        // Устанавливаем индексы тайлов в RGBA-формате
        self.map_handler.set_tile_indices(map_indices, map_width)?;
        self.is_map_updated = true;
        Ok(())
    }

    /// Обновляет MVP-матрицу
    pub fn update_mvp_matrix(&self, gcx: &WgpuGraphContext, mvp_matrix: [[f32; 4]; 4]) {
        gcx.queue.write_buffer(
            &self.mvp_matrix_buffer,
            0,
            bytemuck::cast_slice(&[mvp_matrix]),
        );
    }

    /// Возвращает информацию о тайлах (ширина, высота, ширина карты, высота карты)
    pub fn tiles_info(&self) -> (u32, u32, u32, u32) {
        // This method ensures tiles_info_buffer is considered "used"
        // The actual data is managed by the GPU buffer
        let _buffer = &self.tiles_info_buffer; // Reference to ensure it's kept alive
        // Return cached values that match the buffer content
        (
            self.tiles_texture.width,
            self.tiles_texture.height,
            self.map_handler.width(),
            self.map_handler.height(),
        )
    }
}

impl TAsnGuiElement for WgpuMap {
    type GraphContext = WgpuGraphContext;
    type FrameContext = WgpuFrameContext;

    fn update(&mut self, gcx: &Self::GraphContext) {
        if !self.is_map_updated {
            return;
        }

        // Обновляем текстуру напрямую
        self.map_texture.update_from_rgba(
            &gcx.queue,
            bytemuck::cast_slice(self.map_handler.data()),
            self.map_handler.width(),
            self.map_handler.height(),
            AsnTextureFormat::Rgba32Uint,
        );

        self.is_map_updated = false;
    }

    fn draw(&mut self, fcx: &mut Self::FrameContext) {
        // m_trace!("draw");
        let mut render_pass = fcx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &fcx.view,
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

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}

/// Параметры для создания карты тайлов
pub struct MapTilesParams<'a> {
    /// Байты тайлов карты
    pub map_tiles_bytes: &'a [u8],
    /// Ширина тайлов
    pub tiles_width: u32,
    /// Высота тайлов
    pub tiles_height: u32,
}

/// Параметры карты
pub struct MapParams<'a> {
    /// Ширина карты
    pub map_width: u32,
    /// Высота карты
    pub map_height: u32,
    /// Индексы тайлов на карте
    pub tile_indices: &'a [u32],
}

pub fn get_map(
    gcx: &WgpuGraphContext,
    tiles_params: &MapTilesParams,
    map_params: &MapParams,
) -> Result<WgpuMap, Box<dyn std::error::Error>> {
    let device = &gcx.device;
    let queue = &gcx.queue;
    let format = gcx.surface_format;

    // Создаем текстуру тайлов
    let tiles_texture = WgpuTexture::from_bytes(
        device,
        queue,
        tiles_params.map_tiles_bytes,
        "map-tiles-texture.png",
    )?;

    // Создаем uniform-буфер с информацией о тайлах и карте
    let tiles_info_data = [
        tiles_params.tiles_width as f32,
        tiles_params.tiles_height as f32,
        map_params.map_width as f32,
        map_params.map_height as f32,
    ];
    let tiles_info_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Tiles Info Buffer"),
        contents: bytemuck::cast_slice(&tiles_info_data),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    // Создаем uniform-буфер для MVP-матрицы
    let mvp_matrix: [[f32; 4]; 4] = Matrix4::<f32>::identity().into();
    let mvp_matrix_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("MVP Matrix Buffer"),
        contents: bytemuck::cast_slice(&[mvp_matrix]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let mut map_handler = RgbaHandler::new(map_params.map_width, map_params.map_height);
    map_handler.set_tile_indices(map_params.tile_indices, tiles_params.tiles_width)?;

    let map_texture = WgpuTexture::from_rgba(
        device,
        queue,
        bytemuck::cast_slice(map_handler.data()),
        map_handler.width(),
        map_handler.height(),
        "MAP_TEXTURE_0",
        AsnTextureFormat::Rgba32Uint,
    )?;

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
            // Uniform-буфер с информацией о тайлах
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(
                    tiles_info_buffer.as_entire_buffer_binding(),
                ),
            },
            // Текстура тайлов
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&tiles_texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::Sampler(&tiles_texture.sampler),
            },
            // Текстура карты
            wgpu::BindGroupEntry {
                binding: 3,
                resource: wgpu::BindingResource::TextureView(&map_texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 4,
                resource: wgpu::BindingResource::Sampler(&map_texture.sampler),
            },
            // Uniform-буфер для MVP-матрицы
            wgpu::BindGroupEntry {
                binding: 5,
                resource: wgpu::BindingResource::Buffer(
                    mvp_matrix_buffer.as_entire_buffer_binding(),
                ),
            },
        ],
        label: Some("map_diffuse_bind_group"),
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Map Shader"),
        source: wgpu::ShaderSource::Wgsl(SHADER_SOURCE.into()),
    });

    let render_pipeline = get_render_pipeline(device, format, &texture_bind_group_layout, shader);

    let wgpu_map = WgpuMap {
        render_pipeline,
        vertex_buffer,
        index_buffer,
        diffuse_bind_group,
        num_indices,
        map_handler,
        map_texture,
        tiles_texture,
        tiles_info_buffer,
        mvp_matrix_buffer,
        is_map_updated: false,
    };
    Ok(wgpu_map)
}

/// Создает оптимизированную карту с двойной буферизацией
pub fn get_optimized_map(
    gcx: &WgpuGraphContext,
    tiles_params: &MapTilesParams,
    map_params: &MapParams,
) -> Result<OptimizedWgpuMap, Box<dyn std::error::Error>> {
    let device = &gcx.device;
    let queue = &gcx.queue;
    let format = gcx.surface_format;

    // Создаем текстуру тайлов
    let tiles_texture = WgpuTexture::from_bytes(
        device,
        queue,
        tiles_params.map_tiles_bytes,
        "optimized-map-tiles-texture.png",
    )?;

    // Создаем uniform-буфер с информацией о тайлах и карте
    let tiles_info_data = [
        tiles_params.tiles_width as f32,
        tiles_params.tiles_height as f32,
        map_params.map_width as f32,
        map_params.map_height as f32,
    ];
    let tiles_info_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Optimized Tiles Info Buffer"),
        contents: bytemuck::cast_slice(&tiles_info_data),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    // Создаем uniform-буфер для MVP-матрицы
    let mvp_matrix: [[f32; 4]; 4] = Matrix4::<f32>::identity().into();
    let mvp_matrix_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Optimized MVP Matrix Buffer"),
        contents: bytemuck::cast_slice(&[mvp_matrix]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let mut map_handler = RgbaHandler::new(map_params.map_width, map_params.map_height);
    map_handler.set_tile_indices(map_params.tile_indices, tiles_params.tiles_width)?;

    // Создаем две идентичные текстуры карты для двойной буферизации
    let map_texture_0 = WgpuTexture::from_rgba(
        device,
        queue,
        bytemuck::cast_slice(map_handler.data()),
        map_handler.width(),
        map_handler.height(),
        "OPTIMIZED_MAP_TEXTURE_0",
        AsnTextureFormat::Rgba32Uint,
    )?;

    let map_texture_1 = WgpuTexture::from_rgba(
        device,
        queue,
        bytemuck::cast_slice(map_handler.data()),
        map_handler.width(),
        map_handler.height(),
        "OPTIMIZED_MAP_TEXTURE_1",
        AsnTextureFormat::Rgba32Uint,
    )?;

    let map_textures = [map_texture_0, map_texture_1];

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Optimized Map Vertex Buffer"),
        contents: bytemuck::cast_slice(VERTICES),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Optimized Map Index Buffer"),
        contents: bytemuck::cast_slice(INDICES),
        usage: wgpu::BufferUsages::INDEX,
    });

    let num_indices = INDICES.len() as u32;
    let texture_bind_group_layout = get_texture_bind_group_layout(&device);

    // Создаем две bind группы для двойной буферизации
    let diffuse_bind_group_0 = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &texture_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(
                    tiles_info_buffer.as_entire_buffer_binding(),
                ),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&tiles_texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::Sampler(&tiles_texture.sampler),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: wgpu::BindingResource::TextureView(&map_textures[0].view),
            },
            wgpu::BindGroupEntry {
                binding: 4,
                resource: wgpu::BindingResource::Sampler(&map_textures[0].sampler),
            },
            wgpu::BindGroupEntry {
                binding: 5,
                resource: wgpu::BindingResource::Buffer(
                    mvp_matrix_buffer.as_entire_buffer_binding(),
                ),
            },
        ],
        label: Some("optimized_map_diffuse_bind_group_0"),
    });

    let diffuse_bind_group_1 = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &texture_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(
                    tiles_info_buffer.as_entire_buffer_binding(),
                ),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&tiles_texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::Sampler(&tiles_texture.sampler),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: wgpu::BindingResource::TextureView(&map_textures[1].view),
            },
            wgpu::BindGroupEntry {
                binding: 4,
                resource: wgpu::BindingResource::Sampler(&map_textures[1].sampler),
            },
            wgpu::BindGroupEntry {
                binding: 5,
                resource: wgpu::BindingResource::Buffer(
                    mvp_matrix_buffer.as_entire_buffer_binding(),
                ),
            },
        ],
        label: Some("optimized_map_diffuse_bind_group_1"),
    });

    let diffuse_bind_groups = [diffuse_bind_group_0, diffuse_bind_group_1];
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Optimized Map Shader"),
        source: wgpu::ShaderSource::Wgsl(SHADER_SOURCE.into()),
    });

    let render_pipeline = get_render_pipeline(device, format, &texture_bind_group_layout, shader);

    // Создаем staging buffer для асинхронных обновлений
    let staging_buffer_size = (map_handler.width() * map_handler.height() * 4) as u64;
    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Map Staging Buffer"),
        size: staging_buffer_size,
        usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::MAP_WRITE,
        mapped_at_creation: false,
    });

    Ok(OptimizedWgpuMap {
        render_pipeline,
        vertex_buffer,
        index_buffer,
        diffuse_bind_groups,
        num_indices,
        map_handler,
        map_textures,
        current_texture_index: 0,
        tiles_texture,
        tiles_info_buffer,
        mvp_matrix_buffer,
        update_pending: false,
        staging_buffer: Some(staging_buffer),
    })
}

impl OptimizedWgpuMap {
    /// Обновляет карту с новыми индексами тайлов
    pub fn update_map(&mut self, map_indices: &[u32]) -> Result<(), Box<dyn std::error::Error>> {
        let map_width = self.map_handler.width();
        self.map_handler.set_tile_indices(map_indices, map_width)?;
        self.update_pending = true;
        Ok(())
    }

    /// Обновляет MVP-матрицу
    pub fn update_mvp_matrix(&self, gcx: &WgpuGraphContext, mvp_matrix: [[f32; 4]; 4]) {
        gcx.queue.write_buffer(
            &self.mvp_matrix_buffer,
            0,
            bytemuck::cast_slice(&[mvp_matrix]),
        );
    }

    /// Получает информацию о тайлах (ширина, высота, ширина карты, высота карты)
    pub fn tiles_info(&self) -> (u32, u32, u32, u32) {
        (
            self.tiles_texture.width,
            self.tiles_texture.height,
            self.map_handler.width(),
            self.map_handler.height(),
        )
    }

    /// Переключает буферы для двойной буферизации
    fn swap_buffers(&mut self) {
        self.current_texture_index = (self.current_texture_index + 1) % 2;
    }
}

impl TAsnGuiElement for OptimizedWgpuMap {
    type GraphContext = WgpuGraphContext;
    type FrameContext = WgpuFrameContext;

    fn update(&mut self, gcx: &Self::GraphContext) {
        if !self.update_pending {
            return;
        }

        // Используем staging buffer для асинхронного обновления
        if let Some(staging_buffer) = &self.staging_buffer {
            // Копируем данные в staging buffer
            gcx.queue.write_buffer(
                staging_buffer,
                0,
                bytemuck::cast_slice(self.map_handler.data()),
            );

            // Получаем индекс текстуры для обновления (не текущей для рендеринга)
            let update_texture_index = (self.current_texture_index + 1) % 2;

            // Копируем из staging buffer в текстуру
            let mut encoder = gcx
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Optimized Map Update Encoder"),
                });

            let size = wgpu::Extent3d {
                width: self.map_handler.width(),
                height: self.map_handler.height(),
                depth_or_array_layers: 1,
            };

            encoder.copy_buffer_to_texture(
                wgpu::TexelCopyBufferInfo {
                    buffer: staging_buffer,
                    layout: wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(4 * self.map_handler.width()),
                        rows_per_image: Some(self.map_handler.height()),
                    },
                },
                wgpu::TexelCopyTextureInfo {
                    texture: &self.map_textures[update_texture_index].texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                size,
            );

            gcx.queue.submit(std::iter::once(encoder.finish()));

            // Переключаем буферы
            self.swap_buffers();
            self.update_pending = false;
        }
    }

    fn draw(&mut self, fcx: &mut Self::FrameContext) {
        let mut render_pass = fcx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Optimized Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &fcx.view,
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

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(
            0,
            &self.diffuse_bind_groups[self.current_texture_index],
            &[],
        );
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}
