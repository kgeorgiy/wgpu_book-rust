use std::rc::Rc;

use bytemuck::Pod;
use wgpu::{BindGroupLayout, Buffer, Device, PrimitiveState, RenderPipeline, SurfaceConfiguration, TextureFormat, VertexBufferLayout};

use crate::buffer::{TypedBuffer, TypedBufferDescriptor, UntypedBuffer, UntypedBufferDescriptor};
use crate::{RenderConfiguration, CompositeContent, Content, RawWindow};

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

    pub(crate) fn create_typed_buffer<T: Pod, F: Clone>(&self, descriptor: TypedBufferDescriptor<T, F>) -> TypedBuffer<T, F> {
        descriptor.create_buffer(&self)
    }

    pub(crate) fn create_untyped_buffer<F: Clone>(&self, descriptor: &dyn UntypedBufferDescriptor<F>) -> UntypedBuffer<F> {
        descriptor.create_buffer(&self.device)
    }
}

struct Uniform<T, const L: usize> {
    buffer: TypedBuffer<T, ()>,
    bind_group: wgpu::BindGroup,
    bind_group_layout: BindGroupLayout,
}

impl<T: Pod, const L: usize> Uniform<T, L> {
    fn new(device: &WebGPUDevice, descriptor: TypedBufferDescriptor<T, ()>) -> Self {
        let bind_group_layout = device.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Uniform Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let buffer = device.create_typed_buffer(descriptor);
        let bind_group = device.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.buffer.buffer.as_entire_binding(),
            }],
            label: Some("Uniform Bind Group"),
        });
        Uniform { buffer, bind_group, bind_group_layout }
    }
}

pub(crate) struct WebGPUContent {
    device: WebGPUDevice,
    render_pipeline: RenderPipeline,
    vertices: u32,
    vertex_buffers: Vec<Rc<Buffer>>,
    index_buffer: Option<UntypedBuffer<wgpu::IndexFormat>>,
    uniform: Option<Uniform<[[f32; 4]; 4], 1>>,
}

impl<'a> WebGPUContent {
    pub fn new(window: &dyn RawWindow, conf: RenderConfiguration<'a>) -> Box<dyn Content> {
        pollster::block_on(Self::new_async(window, conf))
    }

    pub async fn new_async(window: &dyn RawWindow, conf: RenderConfiguration<'a>) -> Box<dyn Content> {
        let device = WebGPUDevice::new(window).await;

        let vertex_buffers = conf.vertex_buffers.iter()
            .map(|buffer| device.create_untyped_buffer(buffer.as_ref()))
            .collect::<Vec<_>>();
        let index_buffer = conf.index_buffer
            .map(|buffer| device.create_untyped_buffer(buffer.as_ref()));
        let uniform: Option<Uniform<[[f32; 4]; 4], 1>> = conf.uniform_buffer
            .map(|descriptor| Uniform::new(&device, descriptor));

        let bind_group_layouts: Vec<&BindGroupLayout> = uniform.as_ref()
            .map_or(vec![], |u| vec![&u.bind_group_layout]);
        let render_pipeline = Self::create_pipeline(
            &device.device,
            device.texture_format,
            &vertex_buffers.iter()
                .map(|buffer| buffer.format.clone())
                .collect::<Vec<_>>()[..],
            &bind_group_layouts[..],
            conf.shader_source,
            PrimitiveState {
                topology: conf.topology,
                strip_index_format: conf.strip_index_format,
                ..Default::default()
            }
        );

        let mut vertex_buffers_2: Vec<Rc<Buffer>> = Vec::with_capacity(vertex_buffers.len());
        for buffer in vertex_buffers {
            vertex_buffers_2.push(buffer.buffer)
        }

        let uniform_buffer = uniform.as_ref().map(|u| u.buffer.clone());

        let content = Box::new(WebGPUContent {
            device,
            render_pipeline,
            vertex_buffers: vertex_buffers_2,
            index_buffer,
            vertices: conf.vertices as u32,
            uniform,
        });

        match uniform_buffer {
            Some(buffer) => {
                let parts = [
                    (conf.content)(buffer.writer.clone()),
                    content
                ];
                Box::new(CompositeContent::from(parts))
            }
            None => content,
        }
    }

    fn create_pipeline(
        device: &Device,
        format: TextureFormat,
        vertex_buffer_layouts: &'a [VertexBufferLayout<'a>],
        bind_group_layouts: &[&BindGroupLayout],
        shader_source: &str,
        primitive: PrimitiveState
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
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        })
    }
}

impl<'a> Content for WebGPUContent {
    fn resize(&mut self, width: u32, height: u32) {
        self.device.resize(width, height);
    }

    fn redraw(&mut self) {
        let frame = self.device.surface.get_current_texture().expect("Current texture");
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.05, g: 0.062, b: 0.08, a: 1.0 }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            for (slot, buffer) in self.vertex_buffers.iter().enumerate() {
                render_pass.set_vertex_buffer(slot as u32, buffer.slice(..));
            }
            if let Some(uniform) = self.uniform.as_ref() {
                render_pass.set_bind_group(0, &uniform.bind_group, &[]);
            }
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
