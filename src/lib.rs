use wgpu::{IndexFormat, ShaderStages, VertexBufferLayout};

pub use crate::buffer::*;
pub use crate::window_api::*;

pub mod buffer;
pub mod transforms;
mod webgpu;
pub mod window;
mod window_api;

pub struct RenderConfiguration<'a> {
    pub shader_source: &'a str,
    pub vertices: usize,
    pub topology: wgpu::PrimitiveTopology,
    pub strip_index_format: Option<IndexFormat>,
    pub vertex_buffers: &'a [SmartBufferDescriptor<'a, VertexBufferLayout<'static>>],
    pub index_buffer: Option<SmartBufferDescriptor<'a, IndexFormat>>,
    pub uniform_buffers: &'a [SmartBufferDescriptor<'a, ShaderStages>],
    pub content: Box<dyn FnOnce(Vec<BufferWriter>) -> Box<dyn Content>>,
}

impl<'a> Default for RenderConfiguration<'a> {
    fn default() -> Self {
        RenderConfiguration {
            shader_source: "",
            vertices: 0,
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            vertex_buffers: &[],
            index_buffer: None,
            uniform_buffers: &[],
            content: Box::new(|_| Box::new(NoContent)),
        }
    }
}

pub fn run_wgpu<'a>(window_config: &WindowConfiguration, render_config: RenderConfiguration<'a>) -> ! {
    window::show(window_config, move |window| {
        webgpu::WebGPUContent::new(window, render_config)
    })
}
