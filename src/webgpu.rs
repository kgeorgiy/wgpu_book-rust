use std::rc::Rc;
use std::time::Duration;

use wgpu::{BindGroupLayout, Buffer, Device, PrimitiveState, RenderPipeline, ShaderStages, SurfaceConfiguration, TextureFormat, VertexBufferLayout};

use crate::{CompositeContent, Content, RawWindow, RenderConfiguration, SmartBuffer, SmartBufferDescriptor};

pub(crate) struct WebGPUDevice {
    surface: wgpu::Surface,
    surface_config: SurfaceConfiguration,
    pub(crate) device: Device,
    pub(crate) queue: Rc<wgpu::Queue>,
    texture_format: TextureFormat,
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
        let surface_config = SurfaceConfiguration {
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

struct UniformGroup {
    buffers: Vec<SmartBuffer<ShaderStages>>,
    bind_group: wgpu::BindGroup,
    bind_group_layout: BindGroupLayout,
}

impl UniformGroup {
    fn new(device: &WebGPUDevice, descriptors: &[SmartBufferDescriptor<ShaderStages>]) -> Self {
        let buffers: Vec<SmartBuffer<ShaderStages>> = descriptors.iter()
            .map(|descriptor| descriptor.create_buffer(device))
            .collect();

        let layouts: Vec<wgpu::BindGroupLayoutEntry> = buffers.iter().enumerate()
            .map(|(index, buffer)| wgpu::BindGroupLayoutEntry {
                binding: index as u32,
                visibility: buffer.format.clone(),
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            })
            .collect();

        let bind_group_layout =
            device.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Uniform Bind Group Layout"),
                entries: &layouts,
            });

        let entries: Vec<wgpu::BindGroupEntry> = buffers.iter().enumerate()
            .map(|(index, buffer)| wgpu::BindGroupEntry {
                binding: index as u32,
                resource: buffer.buffer.as_entire_binding(),
            })
            .collect();

        let bind_group = device.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &entries,
            label: Some("Uniform Bind Group"),
        });

        Self { buffers, bind_group, bind_group_layout }
    }
}

pub(crate) struct WebGPUContent {
    device: WebGPUDevice,
    render_pipeline: RenderPipeline,
    vertices: u32,
    vertex_buffers: Vec<Rc<Buffer>>,
    index_buffer: Option<SmartBuffer<wgpu::IndexFormat>>,
    uniform_group: UniformGroup,
}

impl<'a> WebGPUContent {
    pub fn new(window: &dyn RawWindow, conf: RenderConfiguration<'a>) -> Box<dyn Content> {
        pollster::block_on(Self::new_async(window, conf))
    }

    pub async fn new_async(
        window: &dyn RawWindow,
        conf: RenderConfiguration<'a>,
    ) -> Box<dyn Content> {
        let device = WebGPUDevice::new(window).await;

        let vertex_buffers = conf.vertex_buffers.iter()
            .map(|descriptor| descriptor.create_buffer(&device))
            .collect::<Vec<_>>();
        let index_buffer = conf.index_buffer
            .map(|descriptor| descriptor.create_buffer(&device));
        let uniform_group = UniformGroup::new(&device, conf.uniform_buffers);

        let render_pipeline = Self::create_pipeline(
            &device.device,
            device.texture_format,
            &vertex_buffers
                .iter()
                .map(|buffer| buffer.format.clone())
                .collect::<Vec<_>>()[..],
            &vec![&uniform_group.bind_group_layout],
            conf.shader_source,
            PrimitiveState {
                topology: conf.topology,
                strip_index_format: conf.strip_index_format,
                cull_mode: conf.cull_mode,
                ..Default::default()
            },
        );

        let mut vertex_buffers_2: Vec<Rc<Buffer>> = Vec::with_capacity(vertex_buffers.len());
        for buffer in vertex_buffers {
            vertex_buffers_2.push(buffer.buffer)
        }

        let uniform_writers = uniform_group.buffers.iter()
            .map(|buffer| buffer.writer.clone())
            .collect::<Vec<_>>();

        let content = Box::new(WebGPUContent {
            device,
            render_pipeline,
            vertex_buffers: vertex_buffers_2,
            index_buffer,
            vertices: conf.vertices as u32,
            uniform_group,
        });

        Box::new(CompositeContent::from([(conf.content)(uniform_writers), content]))
    }

    fn create_pipeline(
        device: &Device,
        format: TextureFormat,
        vertex_buffer_layouts: &'a [VertexBufferLayout<'a>],
        bind_group_layouts: &[&BindGroupLayout],
        shader_source: &str,
        primitive: PrimitiveState,
    ) -> RenderPipeline {
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
                format: TextureFormat::Depth24Plus,
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
                format: TextureFormat::Depth24Plus,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                label: None,
                view_formats: &[TextureFormat::Depth24Plus],
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
            render_pass.set_bind_group(0, &self.uniform_group.bind_group, &[]);
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
