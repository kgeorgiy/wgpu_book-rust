use wgpu::{IndexFormat, ShaderStages, VertexBufferLayout};

pub use crate::buffer::*;
pub use crate::bindings::TextureInfo;
pub use crate::window_api::*;

pub mod buffer;
pub mod transforms;
mod webgpu;
pub mod window;
mod window_api;
mod bindings;

pub struct RenderConfiguration {
    pub shader_source: String,
    pub vertices: usize,
    pub topology: wgpu::PrimitiveTopology,
    pub cull_mode: Option<wgpu::Face>,
    pub strip_index_format: Option<IndexFormat>,
    pub vertex_buffers: Vec<SmartBufferDescriptor<VertexBufferLayout<'static>>>,
    pub index_buffer: Option<SmartBufferDescriptor<IndexFormat>>,
    pub uniform_buffers: Vec<SmartBufferDescriptor<ShaderStages>>,
    pub textures: Vec<TextureInfo>,
    pub content: Box<dyn ContentFactory>,
}

impl RenderConfiguration {
    pub fn run_title(self, title: &str) -> ! {
        run_wgpu_title(title, self);
    }
}

pub trait ContentFactory {
    fn create(&self, uniforms: Vec<BufferWriter>) -> Box<dyn Content>;
}

struct NoContentFactory;

impl ContentFactory for NoContentFactory {
    fn create(&self, _uniforms: Vec<BufferWriter>) -> Box<dyn Content> {
        Box::new(NoContent)
    }
}

impl<'a> Default for RenderConfiguration {
    fn default() -> Self {
        RenderConfiguration {
            shader_source: "".to_string(),
            vertices: 0,
            topology: wgpu::PrimitiveTopology::TriangleList,
            cull_mode: Some(wgpu::Face::Back),
            strip_index_format: None,
            vertex_buffers: vec![],
            index_buffer: None,
            uniform_buffers: vec![],
            textures: vec![],
            content: Box::new(NoContentFactory),
        }
    }
}

pub fn run_wgpu<'a>(window_config: &WindowConfiguration, render_config: RenderConfiguration) -> ! {
    window::show(window_config, move |window| {
        webgpu::WebGPUContent::new(window, render_config).expect("Valid configuration")
    })
}


pub fn run_wgpu_title<'a>(title: &str, render_config: RenderConfiguration) -> ! {
    window::show(&WindowConfiguration { title }, move |window| {
        webgpu::WebGPUContent::new(window, render_config).expect("Valid configuration")
    })
}
