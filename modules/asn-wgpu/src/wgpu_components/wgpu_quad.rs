use crate::{data::VERTICES, wgpu_utils::get_render_pipeline};
use asn_logger::trace;
use wgpu::util::DeviceExt;

const LOG_MODULE_NAME: &str = "wgpu_quad";

pub struct WgpuQuad {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
}

impl WgpuQuad {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat, shader_source: &str) -> Self {
        let render_pipeline = get_render_pipeline(device, format, shader_source);

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        WgpuQuad {
            render_pipeline,
            vertex_buffer,
        }
    }
    pub fn draw(&self, render_pass: &mut wgpu::RenderPass) {
        trace(LOG_MODULE_NAME, format!("draw").as_str());
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..3, 0..1);
    }
}
