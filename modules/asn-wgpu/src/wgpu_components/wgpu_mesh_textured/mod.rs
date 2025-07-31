mod data;
mod utils;
mod vertex;

use crate::texture;
use asn_logger::trace;
use data::{INDICES, LOG_MODULE_NAME, VERTICES};
use utils::get_render_pipeline;
use wgpu::util::DeviceExt;

use crate::wgpu_utils::get_texture_bind_group_layout;

pub struct WgpuQuadTextured {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    diffuse_bind_group: wgpu::BindGroup,
    num_indices: u32,
}

impl WgpuQuadTextured {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        shader_source: &str,
        texture_bytes: &[u8],
    ) -> Self {
        let diffuse_texture =
            texture::Texture::from_bytes(&device, &queue, texture_bytes, "happy-tree.png").unwrap();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
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
            ],
            label: Some("diffuse_bind_group"),
        });

        let render_pipeline =
            get_render_pipeline(device, format, &texture_bind_group_layout, shader_source);

        Self {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            diffuse_bind_group,
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
}
