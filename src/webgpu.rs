use cgmath::{Matrix4, SquareMatrix};
use wgpu::{Buffer, Device, IndexFormat, Queue, RenderPipeline, RenderPipelineDescriptor, Surface, TextureFormat, VertexBufferLayout, util::DeviceExt, ShaderStages, BindingType, BufferBindingType, BindGroupDescriptor, BindGroupLayout, BindGroup};

use crate::{RawWindow, RenderConfiguration, Content, TypedBuffer, Context};
use crate::transforms::create_projection;

pub struct WebGPUContent {
    surface_config: wgpu::SurfaceConfiguration,
    surface: Surface,
    device: Device,
    queue: Queue,
    render_pipeline: RenderPipeline,
    vertices: u32,
    vertex_buffers: Vec<Buffer>,
    index_buffer: Option<(Buffer, IndexFormat)>,
    uniform_buffer: Buffer,
    uniform_bind_group: BindGroup,
    context: Context,
}

impl WebGPUContent {
    pub fn new<'a>(window: &dyn RawWindow, conf: RenderConfiguration) -> Self {
        pollster::block_on(Self::new_async(window, conf))
    }

    pub async fn new_async<'a>(window: &dyn RawWindow, conf: RenderConfiguration<'a>) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface: Surface = unsafe { instance.create_surface(&window) }.expect("Create surface");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

        let format = surface.get_capabilities(&adapter).formats[0];
        let config = wgpu::SurfaceConfiguration {
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

        let mvp: Matrix4<f32> = Matrix4::identity();
        let uniform_buffer_descriptor = &TypedBuffer::new(
            "Uniform Buffer",
            AsRef::<[[f32; 4]; 4]>::as_ref(&mvp),
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            (),
        ).descriptor;

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Uniform Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let uniform_buffer = device.create_buffer_init(uniform_buffer_descriptor);
        let uniform_bind_group = device.create_bind_group(&BindGroupDescriptor{
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("Uniform Bind Group"),
        });

        let render_pipeline = Self::create_pipeline(
            &conf,
            &device,
            format,
            &[&uniform_bind_group_layout]
        );

        WebGPUContent {
            surface,
            surface_config: config,
            queue,
            render_pipeline,
            vertex_buffers: conf.vertex_buffers.iter()
                .map(|buffer| device.create_buffer_init(&buffer.descriptor))
                .collect(),
            index_buffer: conf.index_buffer.as_ref()
                .map(|buffer| (device.create_buffer_init(&buffer.descriptor), buffer.format)),
            device,
            vertices: conf.vertices as u32,
            context: conf.context,
            uniform_buffer,
            uniform_bind_group,
        }
    }

    fn create_pipeline(
        conf: &RenderConfiguration,
        device: &Device,
        format: TextureFormat,
        bind_group_layouts: &[&BindGroupLayout]
    ) -> RenderPipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(conf.shader_source)),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &bind_group_layouts,
            push_constant_ranges: &[],
        });
        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &&shader,
                entry_point: "vs_main",
                buffers: &conf.vertex_buffers.iter()
                    .map(|buffer| buffer.format.clone())
                    .collect::<Vec<VertexBufferLayout>>()[..],
            },
            fragment: Some(wgpu::FragmentState {
                module: &&shader,
                entry_point: "fs_main",
                targets: &[Some(format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                topology: conf.topology,
                strip_index_format: conf.strip_index_format,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });
        render_pipeline
    }
}

impl Content for WebGPUContent {
    fn resize(&mut self, width: u32, height: u32) {
        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(&self.device, &self.surface_config);

        let context = &self.context;
        let projection = create_projection(width as f32 / height as f32, context.fovy);
        let mvp = projection * context.view * context.model_transform;
        let mvp_ref: &[f32; 16] = mvp.as_ref();
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(mvp_ref));
    }

    fn redraw(&mut self) {
        let frame = self.surface.get_current_texture().expect("Current texture");
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
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
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            if let Some((buffer, format)) = self.index_buffer.as_ref() {
                render_pass.set_index_buffer(buffer.slice(..), *format);
                render_pass.draw_indexed(0..self.vertices, 0, 0..1);
            } else {
                render_pass.draw(0..self.vertices, 0..1);
            }
        }
        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
