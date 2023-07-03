use core::mem::size_of;
use core::time::Duration;
use core::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use anyhow::Result;

use crate::{CompositeContent, Content, PipelineConfiguration, RawWindow, RenderConfiguration, RenderPassConfiguration, SmartBuffer, usize_as_u32};
use crate::bindings::Textures;
use crate::uniforms::Uniforms;

pub(crate) struct WebGPUDevice {
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: Rc<wgpu::Queue>,
    texture_format: wgpu::TextureFormat,
}

impl WebGPUDevice {
    async fn new(window: &dyn RawWindow) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        // SAFETY: Valid window handle provided
        let surface = unsafe { instance.create_surface(&window) }.expect("Create surface");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

        let format = *surface.get_capabilities(&adapter)
            .formats.get(0).expect("at least one compatible format");
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: 0,
            height: 0,
            present_mode: wgpu::PresentMode::Mailbox,
            alpha_mode: wgpu::CompositeAlphaMode::default(),
            view_formats: vec![],
        };

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .expect("Failed to create device");
        WebGPUDevice {
            surface,
            surface_config,
            device,
            queue: Rc::new(queue),
            texture_format: format,
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(&self.device, &self.surface_config);
    }

    fn create_texture(&self, label: &str, width: u32, height: u32, usage: wgpu::TextureUsages, format: wgpu::TextureFormat) -> wgpu::Texture {
        self.device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage,
            label: Some(label),
            view_formats: &[],
        })
    }
}

//
// WegGPUContent

pub(crate) struct WebGPURender {
    wg: WebGPUDevice,
    render_passes: Vec<RenderPass>,
    save_image: Option<SaveImage>,
}

impl WebGPURender {
    pub fn content<'a>(window: &dyn RawWindow, conf: RenderConfiguration) -> Result<Box<dyn Content + 'a>> {
        pollster::block_on(Self::content_async(window, conf))
    }

    pub async fn content_async<'a>(
        window: &dyn RawWindow,
        conf: RenderConfiguration,
    ) -> Result<Box<dyn Content + 'a>> {
        let wg = WebGPUDevice::new(window).await;

        let (render_passes, contents_2d): (Vec<RenderPass>, Vec<Vec<Box<dyn Content>>>) =
            conf.render_passes.into_iter()
                .map(|render_pass| RenderPass::new(render_pass, &wg))
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .unzip();

        let mut contents: Vec<Box<dyn Content>> = contents_2d.into_iter().flatten().collect();
        let save_image = conf.save_image.map(SaveImage::new);
        contents.push(Box::new(WebGPURender { wg, render_passes, save_image }));

        Ok(Box::new(CompositeContent { parts: contents }))
    }

    fn render(&mut self) {
        self.render_to_surface();

        self.save_image.iter().for_each(|save_image|
            save_image.render(self, self.wg.surface_config.width, self.wg.surface_config.height));
    }

    fn render_to_surface(&self) {
        let frame = self.wg.surface.get_current_texture().expect("Current texture");
        let encoder = self.render_to_texture(&frame.texture);
        self.wg.queue.submit(Some(encoder.finish()));
        frame.present();
    }

    fn render_to_texture(&self, texture: &wgpu::Texture) -> wgpu::CommandEncoder {
        let wg = &self.wg;
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder: wgpu::CommandEncoder = wg.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            for render_pass in &self.render_passes {
                render_pass.render(wg, &mut encoder, &view, texture.width(), texture.height());
            }
        }
        encoder
    }
}

impl Content for WebGPURender {
    fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.wg.resize(width, height);
        }
    }

    fn update(&mut self, _dt: Duration) {
        self.render();
    }
}

//
// RenderPass

struct RenderPass {
    pipelines: Vec<Pipeline>,
    load: wgpu::LoadOp<wgpu::Color>,
    depth: Option<Depth>,
}

impl RenderPass {
    fn new(conf: RenderPassConfiguration, wg: &WebGPUDevice)
        -> Result<(RenderPass, Vec<Box<dyn Content>>)>
    {
        let depth = conf.depth.map(|depth_conf| Depth { format: depth_conf.format });
        let (pipelines, listeners): (Vec<Pipeline>, Vec<Vec<Box<dyn Content>>>) =
            conf.pipelines.into_iter()
                .map(|pipeline| Pipeline::new(
                    pipeline,
                    wg,
                    depth.as_ref().map(Depth::stencil),
                ))
                .collect::<Result<Vec<_>>>()?
                .into_iter().unzip();
        Ok((
            RenderPass { pipelines, load: conf.load, depth },
            listeners.into_iter().flatten().collect(),
        ))
    }

    pub(crate) fn render(
        &self,
        wg: &WebGPUDevice,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        width: u32,
        height: u32
    ) {
        let depth = self.depth.as_ref().map(|depth| depth.begin_render_pass(wg, width, height));
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations { load: self.load, store: true },
            })],
            depth_stencil_attachment: depth.as_ref().map(RuntimeDepth::attachment),
        });

        for pipeline in &self.pipelines {
            pipeline.render(&mut render_pass);
        }
    }
}

struct Depth {
    format: wgpu::TextureFormat,
}

impl Depth {
    fn stencil(&self) -> wgpu::DepthStencilState {
        wgpu::DepthStencilState {
            format: self.format,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::LessEqual,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }
    }

    fn begin_render_pass(&self, wg: &WebGPUDevice, width: u32, height: u32) -> RuntimeDepth {
        let usage = wgpu::TextureUsages::RENDER_ATTACHMENT;
        let texture = wg.create_texture("Depth", width, height, usage, self.format);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        RuntimeDepth { _texture: texture, view }
    }
}

struct RuntimeDepth {
    _texture: wgpu::Texture,
    view: wgpu::TextureView,
}

impl RuntimeDepth {
    fn attachment(&self) -> wgpu::RenderPassDepthStencilAttachment {
        wgpu::RenderPassDepthStencilAttachment {
            view: &self.view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: false,
            }),
            stencil_ops: None,
        }
    }
}

//
// Pipeline

struct Pipeline {
    pipeline: wgpu::RenderPipeline,
    vertices: u32,
    vertex_buffers: Vec<Rc<wgpu::Buffer>>,
    index_buffer: Option<SmartBuffer<wgpu::IndexFormat>>,
    uniform_groups: Vec<wgpu::BindGroup>,
    textures_groups: Vec<wgpu::BindGroup>,
    instances: u32
}

impl Pipeline {
    fn new(
        conf: PipelineConfiguration,
        wg: &WebGPUDevice,
        depth_stencil: Option<wgpu::DepthStencilState>
    ) -> Result<(Pipeline, Vec<Box<dyn Content>>)> {
        let (vertex_buffers, vertex_decls): (Vec<SmartBuffer<wgpu::VertexBufferLayout>>, Vec<String>) =
            conf.vertices.into_iter()
                .map(|(descriptor, decl)|
                    (descriptor.create_buffer(wg), decl))
                .unzip();
        let index_buffer = conf.indices
            .map(|descriptor| descriptor.create_buffer(wg));
        let textures = Textures::new(wg, &conf.textures)?;

        let uniforms = Uniforms::new(conf.uniforms, wg);

        let render_pipeline = Self::create_pipeline(
            &wg.device,
            wg.texture_format,
            &vertex_buffers.iter()
                .map(|buffer| buffer.format.clone())
                .collect::<Vec<_>>(),
            &[&uniforms.variants.layout, &textures.variants.layout],
            format!(
                "{}\n{}\n{}",
                vertex_decls.join("\n"),
                uniforms.declarations,
                conf.shader_source
            ).as_str(),
            wgpu::PrimitiveState {
                topology: conf.topology,
                strip_index_format: conf.strip_index_format,
                cull_mode: conf.cull_mode,
                ..Default::default()
            },
            depth_stencil,
        );

        let pipeline = Pipeline {
            pipeline: render_pipeline,
            vertices: usize_as_u32(conf.vertex_count),
            vertex_buffers: vertex_buffers.into_iter()
                .map(|buffer| buffer.buffer)
                .collect(),
            index_buffer,
            uniform_groups: uniforms.variants.groups,
            textures_groups: textures.variants.groups,
            instances: usize_as_u32(uniforms.instances)
        };
        Ok((pipeline, conf.listeners))
    }

    fn create_pipeline<'a>(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        vertex_buffer_layouts: &'a [wgpu::VertexBufferLayout<'a>],
        bind_group_layouts: &[&wgpu::BindGroupLayout],
        shader_source: &str,
        primitive: wgpu::PrimitiveState,
        depth_stencil: Option<wgpu::DepthStencilState>,
    ) -> wgpu::RenderPipeline {
        // println!("==========\n{shader_source}\n==========");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(shader_source)),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts,
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: vertex_buffer_layouts,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(format.into())],
            }),
            primitive,
            depth_stencil,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        })
    }

    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);

        for (slot, buffer) in self.vertex_buffers.iter().enumerate() {
            render_pass.set_vertex_buffer(usize_as_u32(slot), buffer.slice(..));
        }

        for group in &self.textures_groups {
            render_pass.set_bind_group(1, group, &[]);
        }

        for group in &self.uniform_groups {
            render_pass.set_bind_group(0, group, &[]);

            match self.index_buffer.as_ref() {
                None => render_pass.draw(0..self.vertices, 0..self.instances),
                Some(buffer) => {
                    render_pass.set_index_buffer(buffer.buffer.slice(..), buffer.format);
                    render_pass.draw_indexed(0..self.vertices, 0, 0..self.instances);
                },
            }
        }
    }
}


//
// SaveImage, SaveImageData

struct SaveImage {
    filename: String,
    data: RefCell<Option<SaveImageData>>,
}

struct SaveImageData {
    width: u32,
    height: u32,
    texture: wgpu::Texture,
    buffer: Arc<wgpu::Buffer>,
    padded_bytes_per_row: u32,
}

impl SaveImage {
    fn new(filename: String) -> Self {
        Self { filename, data: RefCell::new(None) }
    }

    #[allow(clippy::pattern_type_mismatch)]
    fn render(&self, renderer: &WebGPURender, width: u32, height: u32) {
        self.update_data(&renderer.wg, width, height);
        if let Some(data) = &*self.data.borrow() {
            data.render(renderer, self.filename.clone());
        }
    }

    #[allow(clippy::pattern_type_mismatch)]
    fn update_data(&self, wg: &WebGPUDevice, width: u32, height: u32) {
        if let Some(ref data) = &*self.data.borrow() {
            if data.width == width && data.height == height {
                return;
            }
        }

        *self.data.borrow_mut() = Some(SaveImageData::new(wg, width, height));
    }
}

impl SaveImageData {
    const U32_SIZE: u32 = size_of::<u32>() as u32;
    const ALIGN: u32 = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;

    fn new(wg: &WebGPUDevice, width: u32, height: u32) -> SaveImageData {
        let texture = wg.create_texture(
            "Save as image",
            width,
            height,
            wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
            wg.texture_format,
        );
        let padded_bytes_per_row = ((width * Self::U32_SIZE - 1) / Self::ALIGN + 1) * Self::ALIGN;

        let buffer = Arc::new(wg.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Export"),
            size: wgpu::BufferAddress::from(padded_bytes_per_row * height),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        }));
        SaveImageData {
            width,
            height,
            texture,
            buffer,
            padded_bytes_per_row,
        }
    }

    fn render(&self, renderer: &WebGPURender, filename: String) {
        let mut encoder = renderer.render_to_texture(&self.texture);

        encoder.copy_texture_to_buffer(
            self.texture.as_image_copy(),
            wgpu::ImageCopyBuffer {
                buffer: &self.buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(self.padded_bytes_per_row),
                    rows_per_image: Some(self.height),
                },
            },
            self.texture.size(),
        );

        let submission_index = renderer.wg.queue.submit(Some(encoder.finish()));

        let width = self.width;
        let height = self.height;
        let padded_bytes_per_row = self.padded_bytes_per_row;
        let buffer = self.buffer.clone();
        buffer.clone().slice(..).map_async(wgpu::MapMode::Read, move |_| {
            let mapped = buffer.slice(..).get_mapped_range();
            Self::save_image(filename, &mapped, width, height, padded_bytes_per_row);
            drop(mapped);
            buffer.unmap();
        });
        renderer.wg.device.poll(wgpu::Maintain::WaitForSubmissionIndex(submission_index));
    }

    #[allow(clippy::indexing_slicing, clippy::identity_op)]
    fn save_image(filename: String, data: &[u8], width: u32, height: u32, padded_bytes_per_row: u32) {
        use image::{ImageBuffer, Rgba};
        let mut pixels = vec![0; (width * height * Self::U32_SIZE) as usize];
        for r in 0..height {
            let row_pad = r * padded_bytes_per_row;
            let row_unpad = r * width * Self::U32_SIZE;
            for c in 0..width {
                let pad = (row_pad + c * Self::U32_SIZE) as usize;
                let unpad = (row_unpad + c * Self::U32_SIZE) as usize;
                pixels[unpad + 0] = data[pad + 2];
                pixels[unpad + 1] = data[pad + 1];
                pixels[unpad + 2] = data[pad + 0];
                pixels[unpad + 3] = data[pad + 3];
            }
        }
        ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, pixels)
            .expect("image created")
            .save(filename)
            .expect("image saved");
    }
}
