use boxed::FuncBox;

pub use crate::bindings::TextureInfo;
pub use crate::buffer::*;
pub use crate::uniforms::*;
pub use crate::window_api::*;

pub mod buffer;
pub mod transforms;
pub mod boxed;
mod webgpu;
pub mod window;
mod window_api;
mod bindings;
mod uniforms;

//
// RenderConfiguration

#[must_use]
pub struct RenderConfiguration {
    render_passes: Vec<RenderPassConfiguration>,
    save_image: Option<String>,
}

impl RenderConfiguration {
    pub fn new() -> Self {
        Self { render_passes: vec![], save_image: None }
    }

    #[allow(clippy::indexing_slicing)]
    pub fn new_pass(&mut self, pipelines: Vec<PipelineConfiguration>) -> &mut RenderPassConfiguration {
        let pass = RenderPassConfiguration::new(pipelines);
        self.render_passes.push(pass);
        let last = self.render_passes.len() - 1;
        &mut self.render_passes[last]
    }

    pub fn add_pass(&mut self, pass: RenderPassConfiguration) -> &mut Self {
        self.render_passes.push(pass);
        self
    }

    pub fn save_images_as(&mut self, filename: &str) -> &mut Self {
        self.save_image = Some(filename.to_owned());
        self
    }

    pub fn run_title(self, title: &str) -> ! {
        run_wgpu(&WindowConfiguration { title }, self);
    }
}

impl Default for RenderConfiguration {
    fn default() -> Self {
        Self::new()
    }
}


//
// RenderPassConfiguration

#[must_use]
pub struct RenderPassConfiguration {
    pipelines: Vec<PipelineConfiguration>,
    load: wgpu::LoadOp<wgpu::Color>,
    depth: Option<DepthConfiguration>,
}

impl RenderPassConfiguration {
    pub fn run_title(self, title: &str) -> ! {
        let mut render = RenderConfiguration::new();
        render.add_pass(self);
        render.run_title(title)
    }

    pub fn new(pipelines: Vec<PipelineConfiguration>) -> Self {
        Self {
            pipelines,
            load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.05, g: 0.062, b: 0.08, a: 1.0 }),
            depth: Some(DepthConfiguration { format: wgpu::TextureFormat::Depth24Plus }),
        }
    }

    pub fn with_load(&mut self, load: wgpu::LoadOp<wgpu::Color>) -> &mut Self {
        self.load = load;
        self
    }

    pub fn with_depth(&mut self, format: Option<wgpu::TextureFormat>) -> &mut Self {
        self.depth = format.map(|frm| DepthConfiguration { format: frm });
        self
    }
}

//
// PipelineConfiguration

#[must_use]
pub struct PipelineConfiguration {
    shader_source: String,
    vertex_count: usize,
    topology: wgpu::PrimitiveTopology,
    cull_mode: Option<wgpu::Face>,
    strip_index_format: Option<wgpu::IndexFormat>,
    vertices: Vec<(SmartBufferDescriptor<wgpu::VertexBufferLayout<'static>>, String)>,
    indices: Option<SmartBufferDescriptor<wgpu::IndexFormat>>,
    uniforms: UniformsConfiguration,
    listeners: Vec<Box<dyn Content<()>>>,
    textures: Vec<TextureInfo>,
}

impl PipelineConfiguration {
    pub fn new(shader_source: &str) -> Self {
        PipelineConfiguration {
            shader_source: shader_source.to_owned(),
            vertex_count: 0,
            topology: wgpu::PrimitiveTopology::TriangleList,
            cull_mode: Some(wgpu::Face::Back),
            strip_index_format: None,
            vertices: vec![],
            listeners: vec![],
            indices: None,
            uniforms: UniformsConfiguration::default(),
            textures: vec![],
        }
    }

    pub fn with_shader(mut self, shader_source: &str) -> Self {
        self.shader_source = shader_source.to_owned();
        self
    }

    pub fn with_indexed_vertices<V, I>(mut self, vertices: Vec<V>, indices: &[I], topology: wgpu::PrimitiveTopology)
        -> Self where V: VertexBufferInfo, I: IndexBufferInfo
    {
        self.indices = Some(I::buffer("Indices", indices));
        self
            .with_vertices(vertices, topology)
            .with_vertex_count(indices.len())
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn with_vertices<V: VertexBufferInfo>(
        mut self,
        vertices: Vec<V>,
        topology: wgpu::PrimitiveTopology
    ) -> Self {
        self.vertices = vec![(V::buffer("Vertices", &vertices), V::struct_declaration())];
        self
            .with_topology(topology)
            .with_vertex_count(vertices.len())
    }

    pub fn with_vertices_indices<V, I>(self, vertices: Vec<V>, indices: Option<&[I]>, topology: wgpu::PrimitiveTopology)
        -> Self where V: VertexBufferInfo, I: IndexBufferInfo
    {
        match indices {
            None => self.with_vertices(vertices, topology),
            Some(idx) => self.with_indexed_vertices(vertices, idx, topology),
        }
    }

    fn with_topology(mut self, topology: wgpu::PrimitiveTopology) -> Self {
        self.topology = topology;
        self
    }

    pub fn with_full_topology(
        mut self,
        topology: wgpu::PrimitiveTopology,
        strip_index_format: Option<wgpu::IndexFormat>,
    ) -> Self {
        self.strip_index_format = strip_index_format;
        self.with_topology(topology)
    }


    pub fn with_strip(mut self, topology: wgpu::PrimitiveTopology) -> Self {
        self.topology = topology;
        self
    }

    pub fn with_textures<const L: usize>(mut self, textures: [TextureInfo; L]) -> Self {
        self.textures = textures.into_iter().collect();
        self
    }

    pub fn with_cull_mode(mut self, cull_mode: Option<wgpu::Face>) -> Self {
        self.cull_mode = cull_mode;
        self
    }

    pub fn add_listener(&mut self, listener: Box<dyn Content<()>>) -> &mut Self {
        self.listeners.push(listener);
        self
    }

    pub fn with_vertex_count(mut self, vertices: usize) -> Self {
        self.vertex_count = vertices;
        self
    }

    pub fn uniforms(&mut self) -> &mut UniformsConfiguration {
        &mut self.uniforms
    }

    pub fn with(self, configurator: Configurator<Self>) -> Self {
        configurator.apply(self)
    }

    pub fn run_title(self, title: &str) -> ! {
        RenderPassConfiguration::new(vec![self]).run_title(title);
    }
}

//
// DepthConfiguration

struct DepthConfiguration {
    format: wgpu::TextureFormat,
}

pub type Configurator<T> = FuncBox<T, T>;


pub fn run_wgpu(window_config: &WindowConfiguration, render_config: RenderConfiguration) -> ! {
    window::show(window_config, move |window| {
        webgpu::WebGPURender::content(window, render_config).expect("Valid configuration")
    })
}

fn usize_as_u32(size: usize) -> u32 {
    u32::try_from(size).expect("Size/index should fit into u32")
}
