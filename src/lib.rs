use bytemuck::Pod;
use wgpu::{IndexFormat, VertexBufferLayout};

pub use crate::buffer::*;
pub use crate::window_api::*;

pub mod buffer;
pub mod transforms;
mod webgpu;
pub mod window;
mod window_api;

pub struct RenderConfiguration<'a, T> {
    pub shader_source: &'a str,
    pub vertices: usize,
    pub topology: wgpu::PrimitiveTopology,
    pub strip_index_format: Option<IndexFormat>,
    pub vertex_buffers: &'a [Box<dyn UntypedBufferDescriptor<VertexBufferLayout<'static>> + 'a>],
    pub index_buffer: Option<Box<dyn UntypedBufferDescriptor<IndexFormat> + 'a>>,
    pub uniform_buffer: Option<TypedBufferDescriptor<'a, T, ()>>,
    pub content: Box<dyn FnOnce(BufferWriter<T>) -> Box<dyn Content>>,
}

impl<'a, T: Pod> Default for RenderConfiguration<'a, T> {
    fn default() -> Self {
        RenderConfiguration {
            shader_source: "",
            vertices: 0,
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            vertex_buffers: &[],
            index_buffer: None,
            uniform_buffer: None,
            content: Box::new(|_| Box::new(NoContent)),
        }
    }
}

pub fn run_wgpu<T: Pod>(window_config: &WindowConfiguration, render_config: RenderConfiguration<T>) -> ! {
    window::show(window_config, move |window| {
        webgpu::WebGPUContent::new(window, render_config)
    })
}
