mod data;
mod utils;
mod vertex;

use crate::{texture, wgpu_components::wgpu_map::utils::get_texture_bind_group_layout};
use asn_logger::trace;
use data::{BLUE_PIXEL, INDICES, LOG_MODULE_NAME, VERTICES};
use utils::get_render_pipeline;
use wgpu::util::DeviceExt;

pub struct WgpuMap {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    diffuse_bind_group: wgpu::BindGroup,
    num_indices: u32,
    map_texture: texture::Texture,
}

impl WgpuMap {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        shader_source: &str,
        texture_bytes: &[u8],
    ) -> Self {
        let diffuse_texture =
            texture::Texture::from_bytes(&device, &queue, texture_bytes, "map-texture.png")
                .unwrap();

        let map_texture =
            texture::Texture::from_rgba(&device, &queue, BLUE_PIXEL, 1, 1, "BLUE_PIXEL").unwrap();

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

        let render_pipeline =
            get_render_pipeline(device, format, &texture_bind_group_layout, shader_source);

        Self {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            diffuse_bind_group,
            map_texture,
        }
    }

    pub fn update_map(&self, queue: &wgpu::Queue, rgba: &[u8], width: u32, height: u32) {
        self.map_texture
            .update_from_rgba(queue, rgba, width, height);
    }

    pub fn draw(&self, render_pass: &mut wgpu::RenderPass) {
        trace(LOG_MODULE_NAME, format!("draw").as_str());
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}
