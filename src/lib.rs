pub use crate::bindings::TextureInfo;
pub use crate::buffer::*;
pub use crate::window_api::*;
pub use crate::uniforms::*;

pub mod buffer;
pub mod transforms;
mod webgpu;
pub mod window;
mod window_api;
mod bindings;
mod uniforms;

// pub struct RenderConfiguration {
//     pub pipelines: Vec<PipelineConfiguration<>>
// }


pub struct RenderConfiguration {
    pub _placeholder: (), // Waiting for #[non_exhaustive(pub)]
    pub shader_source: String,
    pub vertices: usize,
    pub topology: wgpu::PrimitiveTopology,
    pub cull_mode: Option<wgpu::Face>,
    pub strip_index_format: Option<wgpu::IndexFormat>,
    pub vertex_buffers: Vec<SmartBufferDescriptor<wgpu::VertexBufferLayout<'static>>>,
    pub index_buffer: Option<SmartBufferDescriptor<wgpu::IndexFormat>>,
    pub uniforms: Option<UniformsConfiguration>,
    pub textures: Vec<TextureInfo>,
    pub instances: usize,
}

impl RenderConfiguration {
    pub fn run_title(self, title: &str) -> ! {
        window::show(&WindowConfiguration { title }, move |window| {
            webgpu::WebGPUContent::content(window, self).expect("Valid configuration")
        });
    }
}

impl Default for RenderConfiguration {
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
            instances: 1,
        }
    }
}


pub fn run_wgpu(window_config: &WindowConfiguration, render_config: RenderConfiguration) -> ! {
    window::show(window_config, move |window| {
        webgpu::WebGPUContent::content(window, render_config).expect("Valid configuration")
    })
}

fn usize_as_u32(size: usize) -> u32 {
    u32::try_from(size).expect("Size/index should fit into u32")
}
