use std::mem::size_of;
use bytemuck::{cast_slice, Pod};
use cgmath::{Matrix4, Rad, SquareMatrix};
use wgpu::{BufferUsages, IndexFormat, VertexAttribute, VertexBufferLayout, VertexStepMode};
use wgpu::util::BufferInitDescriptor;

pub mod window;
pub mod transforms;
mod webgpu;

pub trait Content {
    fn resize(&mut self, width: u32, height: u32);
    fn redraw(&mut self);
}

impl Content for NoContent {
    fn resize(&mut self, _width: u32, _height: u32) {
    }

    fn redraw(&mut self) {
    }
}

pub struct NoContent;

pub struct WindowConfiguration<'a> {
    pub title: &'a str,
}

pub struct TypedBuffer<'a, F> {
    descriptor: BufferInitDescriptor<'a>,
    format: F,
}

impl<'a, F> TypedBuffer<'a, F> {
    fn new<T: Pod>(label: &'a str, items: &'a [T], usage: BufferUsages, format: F) -> Self {
        TypedBuffer {
            format,
            descriptor: BufferInitDescriptor {
                label: Some(label),
                contents: cast_slice(items),
                usage,
            }
        }
    }
}

pub trait VertexBufferInfo where Self: Pod {
    const ATTRIBUTES: &'static [VertexAttribute];
    const USAGE: BufferUsages = BufferUsages::VERTEX;

    fn format() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: size_of::<Self>() as wgpu::BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }

    fn buffer<'a>(label: &'a str, vertices: &'a [Self]) -> TypedBuffer<'a, VertexBufferLayout<'static>> {
        TypedBuffer::new(label, vertices, Self::USAGE, Self::format())
    }
}

pub trait BufferInfo<T> where Self: Pod {
    const USAGE: BufferUsages;

    fn format() -> T;

    fn buffer<'a>(label: &'a str, items: &'a [Self]) -> TypedBuffer<'a, T> {
        TypedBuffer::new(label, items, Self::USAGE, Self::format())
    }
}

pub struct BufferUtil;


impl BufferInfo<IndexFormat> for u16 {
    const USAGE: BufferUsages = BufferUsages::INDEX;

    fn format<'a>() -> IndexFormat {
        IndexFormat::Uint16
    }
}


pub struct RenderConfiguration<'a> {
    pub shader_source: &'a str,
    pub vertices: usize,
    pub topology: wgpu::PrimitiveTopology,
    pub strip_index_format: Option<IndexFormat>,
    pub vertex_buffers: &'a [TypedBuffer<'a, VertexBufferLayout<'a>>],
    pub index_buffer: Option<TypedBuffer<'a, IndexFormat>>,
    pub context: Context,
}

impl<'a> Default for RenderConfiguration<'a> {
    fn default() -> Self {
        RenderConfiguration {
            shader_source: "",
            vertices: 0,
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            vertex_buffers: &[],
            index_buffer: None,
            context: Context {
                model_transform: Matrix4::identity(),
                view: Matrix4::identity(),
                fovy: Rad(0.0),
            },
        }
    }
}

pub struct Context {
    pub model_transform: Matrix4<f32>,
    pub view: Matrix4<f32>,
    pub fovy: Rad<f32>,
}

pub trait RawWindow: raw_window_handle::HasRawWindowHandle + raw_window_handle::HasRawDisplayHandle {}

impl<W> RawWindow for W where W: raw_window_handle::HasRawWindowHandle + raw_window_handle::HasRawDisplayHandle {}

pub fn run_wgpu(window_config: &WindowConfiguration, render_config: RenderConfiguration) -> ! {
    window::show(
        window_config,
        move |window| Box::new(webgpu::WebGPUContent::new(window, render_config)),
    )
}
