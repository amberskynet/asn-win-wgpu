use asn_gui_core::TAsnGuiElement;
use asn_wgpu::render_manager::WgpuContext;
use asn_wgpu::wgpu::util::DeviceExt;
use asn_wgpu::{FrameContext, wgpu};

use crate::data::rgba_handler::RgbaHandler;
use crate::data::texture::WgpuTexture;
use crate::data::utils::{get_render_pipeline, get_texture_bind_group_layout};
use crate::data::{DEFAULT_CLEAR_COLOR, INDICES, SHADER_SOURCE, VERTICES};

mod data;

pub struct WgpuMap {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    diffuse_bind_group: wgpu::BindGroup,
    num_indices: u32,
    map_handler: RgbaHandler,
    map_texture: WgpuTexture,
    is_map_updated: bool,
}

impl TAsnGuiElement for WgpuMap {
    type FrameContext = FrameContext;

    fn update(&mut self) {
        todo!()
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

pub fn get_map(
    gcx: &WgpuContext,
    map_tiles_bytes: &[u8],
    map_width: u32,
    map_height: u32,
) -> WgpuMap {
    let device = &gcx.device;
    let queue = &gcx.queue;
    let format = gcx.surface_format;

    let diffuse_texture =
        WgpuTexture::from_bytes(device, queue, map_tiles_bytes, "map-texture.png").unwrap();

    let mut map_handler = RgbaHandler::new(map_width, map_height);
    map_handler.fill_random();

    let map_texture = WgpuTexture::from_rgba(
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

    let render_pipeline = get_render_pipeline(device, format, &texture_bind_group_layout, shader);

    WgpuMap {
        render_pipeline,
        vertex_buffer,
        index_buffer,
        diffuse_bind_group,
        num_indices,
        map_handler,
        map_texture,
        is_map_updated: false,
    }
}
