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


// RenderConfiguration

pub struct RenderConfiguration {
    pub pipelines: Vec<PipelineConfiguration<>>
}

impl RenderConfiguration {
    pub fn run_title(self, title: &str) -> ! {
        run_wgpu(&WindowConfiguration { title }, self);
    }
}


// PipelineConfiguration

pub struct PipelineConfiguration {
    shader_source: String,
    vertex_count: usize,
    topology: wgpu::PrimitiveTopology,
    cull_mode: Option<wgpu::Face>,
    strip_index_format: Option<wgpu::IndexFormat>,
    vertices: Vec<SmartBufferDescriptor<wgpu::VertexBufferLayout<'static>>>,
    indices: Option<SmartBufferDescriptor<wgpu::IndexFormat>>,
    uniforms: Option<UniformsConfiguration>,
    textures: Vec<TextureInfo>,
    instances: usize,
}

impl PipelineConfiguration {
    #[must_use] pub fn new(shader_source: &str) -> Self {
        PipelineConfiguration {
            shader_source: shader_source.to_owned(),
            vertex_count: 0,
            topology: wgpu::PrimitiveTopology::TriangleList,
            cull_mode: Some(wgpu::Face::Back),
            strip_index_format: None,
            vertices: vec![],
            indices: None,
            uniforms: None,
            textures: vec![],
            instances: 1,
        }
    }

    #[must_use] pub fn with_shader(mut self, shader_source: &str) -> Self {
        self.shader_source = shader_source.to_owned();
        self
    }

    #[must_use] pub fn with_indexed_vertices<V, I>(mut self, vertices: &[V], indices: &[I])
        -> Self where V: VertexBufferInfo, I: IndexBufferInfo
    {
        self.vertices = vec![V::buffer("Vertices", vertices)];
        self.indices = Some(I::buffer("Indices", indices));
        self.with_vertex_count(indices.len())
    }

    #[must_use] pub fn with_vertices<V: VertexBufferInfo>(mut self, vertices: &[V]) -> Self {
        self.vertices = vec![V::buffer("Vertices", vertices)];
        self.with_vertex_count(vertices.len())
    }

    #[must_use] pub fn with_vertices_indices<V, I>(self, vertices: &[V], indices: Option<&[I]>)
        -> Self where V: VertexBufferInfo, I: IndexBufferInfo
    {
        match indices {
            None => self.with_vertices(vertices),
            Some(idx) => self.with_indexed_vertices(vertices, idx),
        }
    }

    #[must_use] pub fn with_topology(mut self, topology: wgpu::PrimitiveTopology) -> Self {
        self.topology = topology;
        self
    }

    #[must_use] pub fn with_full_topology(
        mut self,
        topology: wgpu::PrimitiveTopology,
        strip_index_format: Option<wgpu::IndexFormat>,
    ) -> Self {
        self.strip_index_format = strip_index_format;
        self.with_topology(topology)
    }


    #[must_use] pub fn with_strip(mut self, topology: wgpu::PrimitiveTopology) -> Self {
        self.topology = topology;
        self
    }

    #[must_use] pub fn with_textures<const L: usize>(mut self, textures: [TextureInfo; L]) -> Self {
        self.textures = textures.into_iter().collect();
        self
    }

    #[must_use] pub fn with_cull_mode(mut self, cull_mode: Option<wgpu::Face>) -> Self {
        self.cull_mode = cull_mode;
        self
    }

    #[must_use] pub fn with_uniforms<const L: usize>(
        self,
        buffers: [SmartBufferDescriptor<wgpu::ShaderStages>; L],
        content_factory: Box<dyn ContentFactory<L>>
    ) -> Self {
        self.with_multi_uniforms(buffers, content_factory, vec!([0; L]))
    }

    #[must_use] pub fn with_multi_uniforms<const UL: usize>(
        mut self,
        buffers: [SmartBufferDescriptor<wgpu::ShaderStages>; UL],
        content_factory: Box<dyn ContentFactory<UL>>,
        variants: Vec<[usize; UL]>
    ) -> Self {
        self.uniforms = Some(UniformsConfiguration::new(buffers, content_factory, variants));
        self
    }

    #[must_use] pub fn with_instances(mut self, instances: usize) -> Self {
        self.instances = instances;
        self
    }

    #[must_use] pub fn with_vertex_count(mut self, vertices: usize) -> Self {
        self.vertex_count = vertices;
        self
    }

    pub fn run_title(self, title: &str) -> ! {
        RenderConfiguration { pipelines: vec![self] }.run_title(title);
    }
}


pub fn run_wgpu(window_config: &WindowConfiguration, render_config: RenderConfiguration) -> ! {
    window::show(window_config, move |window| {
        webgpu::WebGPURender::content(window, render_config).expect("Valid configuration")
    })
}

fn usize_as_u32(size: usize) -> u32 {
    u32::try_from(size).expect("Size/index should fit into u32")
}
