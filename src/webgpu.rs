use std::rc::Rc;
use std::time::Duration;

use anyhow::Result;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::{CompositeContent, Content, RawWindow, RenderConfiguration, SmartBuffer};
use crate::bindings::{Uniforms, Textures};

pub(crate) struct WebGPUDevice {
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: Rc<wgpu::Queue>,
    texture_format: wgpu::TextureFormat,
}

impl WebGPUDevice {
    pub(crate) fn create_buffer_init(&self, descriptor: &BufferInitDescriptor) -> wgpu::Buffer {
        self.device.create_buffer_init(descriptor)
    }
}

impl WebGPUDevice {
    async fn new(window: &dyn RawWindow) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = unsafe { instance.create_surface(&window) }.expect("Create surface");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

        let format = surface.get_capabilities(&adapter).formats[0];
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: 0,
            height: 0,
            present_mode: wgpu::PresentMode::Mailbox,
            alpha_mode: Default::default(),
            view_formats: vec![format],
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

pub(crate) struct WebGPUContent {
    device: WebGPUDevice,
    render_pipeline: wgpu::RenderPipeline,
    vertices: u32,
    vertex_buffers: Vec<Rc<wgpu::Buffer>>,
    index_buffer: Option<SmartBuffer<wgpu::IndexFormat>>,
    bind_groups: Vec<wgpu::BindGroup>,
}

impl WebGPUContent {
    pub fn new<'a>(window: &dyn RawWindow, conf: RenderConfiguration) -> Result<Box<dyn Content + 'a>> {
        pollster::block_on(Self::new_async(window, conf))
    }

    pub async fn new_async<'a>(
        window: &dyn RawWindow,
        conf: RenderConfiguration,
    ) -> Result<Box<dyn Content + 'a>> {
        let device = WebGPUDevice::new(window).await;

        let vertex_buffers = conf.vertex_buffers.iter()
            .map(|descriptor| descriptor.create_buffer(&device))
            .collect::<Vec<_>>();
        let index_buffer = conf.index_buffer
            .map(|descriptor| descriptor.create_buffer(&device));
        let uniforms = Uniforms::new(&device, &conf.uniform_buffers);
        let textures = Textures::new(&device, &conf.textures)?;

        let render_pipeline = Self::create_pipeline(
            &device.device,
            device.texture_format,
            &vertex_buffers.iter()
                .map(|buffer| buffer.format.clone())
                .collect::<Vec<_>>(),
            &vec![&uniforms.bindings.layout, &textures.bindings.layout],
            conf.shader_source.as_str(),
            wgpu::PrimitiveState {
                topology: conf.topology,
                strip_index_format: conf.strip_index_format,
                cull_mode: conf.cull_mode,
                ..Default::default()
            },
        );

        let uniform_writers = uniforms.writers();
        let content = Box::new(WebGPUContent {
            device,
            render_pipeline,
            vertex_buffers: vertex_buffers.into_iter()
                .map(|buffer| buffer.buffer)
                .collect(),
            index_buffer,
            vertices: conf.vertices as u32,
            bind_groups: vec![uniforms.bindings.group, textures.bindings.group],
        });

        Ok(Box::new(CompositeContent::from([conf.content.create(uniform_writers), content])))
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
                module: &&shader,
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

impl<'a> Content for WebGPUContent {
    fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.device.resize(width, height);
        }
    }

    fn update(&mut self, _dt: Duration) {
        let frame = self
            .device
            .surface
            .get_current_texture()
            .expect("Current texture");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let depth_texture = self.device.device.create_texture(&wgpu::TextureDescriptor {
                size: wgpu::Extent3d {
                    width: self.device.surface_config.width,
                    height: self.device.surface_config.height,
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
            render_pass.set_pipeline(&self.render_pipeline);
            for (slot, buffer) in self.vertex_buffers.iter().enumerate() {
                render_pass.set_vertex_buffer(slot as u32, buffer.slice(..));
            }
            self.bind_groups.iter().enumerate()
                .for_each(|(index, group)| render_pass.set_bind_group(index as u32, group, &[]));
            if let Some(buffer) = self.index_buffer.as_ref() {
                render_pass.set_index_buffer(buffer.buffer.slice(..), buffer.format);
                render_pass.draw_indexed(0..self.vertices, 0, 0..1);
            } else {
                render_pass.draw(0..self.vertices, 0..1);
            }
        }
        self.device.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
