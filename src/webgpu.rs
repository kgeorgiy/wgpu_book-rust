use std::rc::Rc;
use core::time::Duration;

use anyhow::Result;

use crate::{CompositeContent, Content, RawWindow, PipelineConfiguration, SmartBuffer, usize_as_u32, RenderConfiguration};
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
}


// WegGPUContent

pub(crate) struct WebGPURender {
    wg: WebGPUDevice,
    pipelines: Vec<Pipeline>,
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

        let (pipelines, mut contents): (Vec<Pipeline>, Vec<Box<dyn Content>>)
            = conf.pipelines.into_iter()
                .map(|pipeline| Self::configure_pipeline(pipeline, &wg))
                .collect::<Result<Vec<_>>>()?
                .into_iter().unzip();

        contents.push(Box::new(WebGPURender { wg, pipelines }));

        Ok(Box::new(CompositeContent { parts: contents }))
    }

    fn configure_pipeline(conf: PipelineConfiguration, wg: &WebGPUDevice)
        -> Result<(Pipeline, Box<dyn Content>)>
    {
        let vertex_buffers = conf.vertices.into_iter()
            .map(|descriptor| descriptor.create_buffer(wg))
            .collect::<Vec<_>>();
        let index_buffer = conf.indices
            .map(|descriptor| descriptor.create_buffer(wg));
        let uniforms = Uniforms::new(wg, conf.uniforms);
        let textures = Textures::new(wg, &conf.textures)?;

        let render_pipeline = Self::create_pipeline(
            &wg.device,
            wg.texture_format,
            &vertex_buffers.iter()
                .map(|buffer| buffer.format.clone())
                .collect::<Vec<_>>(),
            &[&uniforms.variants.layout, &textures.variants.layout],
            conf.shader_source.as_str(),
            wgpu::PrimitiveState {
                topology: conf.topology,
                strip_index_format: conf.strip_index_format,
                cull_mode: conf.cull_mode,
                ..Default::default()
            },
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
            instances: usize_as_u32(conf.instances)
        };
        Ok((pipeline, uniforms.content))
    }

    fn create_pipeline<'a>(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        vertex_buffer_layouts: &'a [wgpu::VertexBufferLayout<'a>],
        bind_group_layouts: &[&wgpu::BindGroupLayout],
        shader_source: &str,
        primitive: wgpu::PrimitiveState,
    ) -> wgpu::RenderPipeline {
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
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        })
    }
}

impl Content for WebGPURender {
    fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.wg.resize(width, height);
        }
    }

    fn update(&mut self, _dt: Duration) {
        let frame = self
            .wg
            .surface
            .get_current_texture()
            .expect("Current texture");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .wg
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let depth_texture = self.wg.device.create_texture(&wgpu::TextureDescriptor {
                size: wgpu::Extent3d {
                    width: self.wg.surface_config.width,
                    height: self.wg.surface_config.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth24Plus,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                label: None,
                view_formats: &[wgpu::TextureFormat::Depth24Plus],
            });
            let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.05,
                            g: 0.062,
                            b: 0.08,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: false,
                    }),
                    stencil_ops: None,
                }),
            });

            for pipeline in &self.pipelines {
                pipeline.render(&mut render_pass);
            }
        }
        self.wg.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}


// Pipeline

struct Pipeline {
    pipeline: wgpu::RenderPipeline,
    vertices: u32,
    vertex_buffers: Vec<wgpu::Buffer>,
    index_buffer: Option<SmartBuffer<wgpu::IndexFormat>>,
    uniform_groups: Vec<wgpu::BindGroup>,
    textures_groups: Vec<wgpu::BindGroup>,
    instances: u32
}

impl Pipeline {
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