pub use crate::bindings::TextureInfo;
pub use crate::buffer::*;
pub use crate::window_api::*;

pub mod buffer;
pub mod transforms;
mod webgpu;
pub mod window;
mod window_api;
mod bindings;

pub struct RenderConfiguration<const UL: usize> {
    pub _placeholder: (), // Waiting for #[non_exhaustive(pub)]
    pub shader_source: String,
    pub vertices: usize,
    pub topology: wgpu::PrimitiveTopology,
    pub cull_mode: Option<wgpu::Face>,
    pub strip_index_format: Option<wgpu::IndexFormat>,
    pub vertex_buffers: Vec<SmartBufferDescriptor<wgpu::VertexBufferLayout<'static>>>,
    pub index_buffer: Option<SmartBufferDescriptor<wgpu::IndexFormat>>,
    pub uniforms: Option<UniformsConfiguration<UL>>,
    pub textures: Vec<TextureInfo>,
}

impl<const UL: usize> RenderConfiguration<UL> {
    pub fn run_title(self, title: &str) -> ! {
        window::show(&WindowConfiguration { title }, move |window| {
            webgpu::WebGPUContent::content(window, self).expect("Valid configuration")
        });
    }
}

impl<const UL: usize> Default for RenderConfiguration<UL> {
    fn default() -> Self {
        RenderConfiguration {
            _placeholder: (),
            shader_source: String::new(),
            vertices: 0,
            topology: wgpu::PrimitiveTopology::TriangleList,
            cull_mode: Some(wgpu::Face::Back),
            strip_index_format: None,
            vertex_buffers: vec![],
            index_buffer: None,
            uniforms: None,
            textures: vec![],
        }
    }
}


// UniformConfiguration and associated types

pub struct UniformsConfiguration<const UL: usize> {
    content_factory: Box<dyn ContentFactory<UL>>,
    buffers: Vec<SmartBufferDescriptor<wgpu::ShaderStages>>,
}

impl<const UL: usize> UniformsConfiguration<UL> {
    #[must_use] pub fn new(
        buffers: [SmartBufferDescriptor<wgpu::ShaderStages>; UL],
        content_factory: Box<dyn ContentFactory<UL>>
    ) -> Option<Self> {
        Some(Self { content_factory, buffers: buffers.into_iter().collect() })
    }
}

pub trait ContentFactory<const UL: usize> {
    fn create(&self, uniforms: [BufferWriter; UL]) -> Box<dyn Content>;

    fn _unsafe_create(&self, uniforms: Vec<BufferWriter>) -> Box<dyn Content> {
        self.create(uniforms.try_into().expect("valid size"))
    }
}

struct NoContentFactory;

impl ContentFactory<0> for NoContentFactory {
    fn create(&self, _uniforms: [BufferWriter; 0]) -> Box<dyn Content> {
        Box::new(NoContent)
    }
}


pub fn run_wgpu<const UL: usize>(window_config: &WindowConfiguration, render_config: RenderConfiguration<UL>) -> ! {
    window::show(window_config, move |window| {
        webgpu::WebGPUContent::content(window, render_config).expect("Valid configuration")
    })
}

fn usize_as_u32(size: usize) -> u32 {
    u32::try_from(size).expect("Size/index should fit into u32")
}
