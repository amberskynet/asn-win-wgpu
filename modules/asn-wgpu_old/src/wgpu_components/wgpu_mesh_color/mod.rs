mod data;
mod utils;
mod vertex;

use asn_logger::m_trace;
use data::{INDICES, LOG_MODULE_NAME, VERTICES};
use utils::get_render_pipeline;
use wgpu::util::DeviceExt;

pub struct WgpuQuad {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

impl WgpuQuad {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat, shader_source: &str) -> Self {
        let render_pipeline = get_render_pipeline(device, format, shader_source);

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

        WgpuQuad {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
        }
    }
    pub fn draw(&self, render_pass: &mut wgpu::RenderPass) {
        m_trace!("draw");
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16); // 1.
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1); // 2.
    }
}
