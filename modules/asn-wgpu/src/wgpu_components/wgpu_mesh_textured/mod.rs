mod data;
mod utils;
mod vertex;

use asn_logger::trace;
use data::{INDICES, LOG_MODULE_NAME, VERTICES};
use utils::get_render_pipeline;
use wgpu::util::DeviceExt;

use crate::wgpu_utils::{get_texture_bind_group, get_texture_bind_group_layout};

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
    ) -> Self {
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

        let diffuse_bind_group =
            get_texture_bind_group(&device, &queue, &texture_bind_group_layout);

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
        render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]); // NEW!
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}
