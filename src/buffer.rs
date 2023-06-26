use std::{any::TypeId, marker::PhantomData, mem::size_of, rc::Rc};

use bytemuck::{cast_slice, Pod};
use wgpu::{Buffer, BufferUsages, IndexFormat, Queue, ShaderStages, VertexAttribute, VertexBufferLayout, VertexStepMode};
use wgpu::util::{BufferInitDescriptor};

use crate::webgpu::WebGPUDevice;

// BufferFormat

#[derive(Clone)]
pub(crate) struct BufferLayout {
    type_id: TypeId,
    len: usize,
}


// SmartBuffer

#[derive(Clone)]
pub(crate) struct SmartBuffer<F: Clone> {
    pub(crate) buffer: Rc<Buffer>,
    pub(crate) writer: BufferWriter,
    pub(crate) format: F,
}

impl<F: Clone> SmartBuffer<F> {}


// BufferWriter

#[derive(Clone)]
pub struct BufferWriter {
    queue: Rc<Queue>,
    buffer: Rc<Buffer>,
    layout: BufferLayout,
}

impl BufferWriter {
    pub fn as_typed<T: 'static>(&self) -> TypedBufferWriter<T> {
        assert_eq!(self.layout.type_id, TypeId::of::<T>());
        TypedBufferWriter { untyped: self.clone(), phantom: Default::default() }
    }

    fn write_slice<T: Pod + 'static>(&self, slice: &[T]) {
        assert_eq!(self.layout.len, slice.len());
        self.queue.write_buffer(&self.buffer, 0, cast_slice(slice));
    }
}

// TypedBufferWriter

#[derive(Clone)]
pub struct TypedBufferWriter<T> {
    untyped: BufferWriter,
    phantom: PhantomData<T>,
}

impl<T: Pod> TypedBufferWriter<T> {
    pub fn write_slice(&self, slice: &[T]) {
        self.untyped.write_slice(slice);
    }

    pub fn write(&self, value: T) {
        self.write_slice(&[value]);
    }
}

// SmartBufferDescriptor

pub struct SmartBufferDescriptor<F: Clone> {
    label: String,
    contents: Vec<u8>,
    usage: BufferUsages,
    // descriptor: BufferInitDescriptor<'a>,
    layout: BufferLayout,
    format: F,
}

impl<'a, F: Clone> SmartBufferDescriptor<F> {
    pub fn new<T: Pod>(label: String, items: &'a [T], usage: BufferUsages, format: F) -> Self {
        Self {
            label,
            contents: cast_slice(items).to_vec(),
            usage,
            layout: BufferLayout {
                len: items.len(),
                type_id: TypeId::of::<T>(),
            },
            format,
        }
    }

    pub(crate) fn create_buffer(&self, device: &WebGPUDevice) -> SmartBuffer<F> {
        let buffer = Rc::new(device.create_buffer_init(&BufferInitDescriptor {
            label: Some(self.label.as_str()),
            contents: &self.contents,
            usage: self.usage,
        }));
        SmartBuffer {
            buffer: buffer.clone(),
            format: self.format.clone(),
            writer: BufferWriter {
                queue: device.queue.clone(),
                buffer: buffer.clone(),
                layout: self.layout.clone(),
            }
        }
    }
}

// BufferInfo

pub trait BufferInfo<F: Clone + 'static> where Self: Pod {
    const USAGE: BufferUsages;
    const FORMAT: F;

    fn buffer(label: &str, items: &[Self]) -> SmartBufferDescriptor<F> {
        Self::buffer_format(label, items, Self::FORMAT.clone())
    }

    fn buffer_format(label: &str, items: &[Self], format: F) -> SmartBufferDescriptor<F> {
        SmartBufferDescriptor::new(label.to_string(), items, Self::USAGE, format)
    }
}

// IndexBufferInfo

pub trait IndexBufferInfo where Self: Pod {
    const FORMAT: IndexFormat;
}

impl<I: IndexBufferInfo> BufferInfo<IndexFormat> for I {
    const USAGE: BufferUsages = BufferUsages::INDEX;
    const FORMAT: IndexFormat = I::FORMAT;
}

impl IndexBufferInfo for u16 {
    const FORMAT: IndexFormat = IndexFormat::Uint16;
}

impl<T: Pod> BufferInfo<ShaderStages> for T {
    const USAGE: BufferUsages = BufferUsages::UNIFORM.union(BufferUsages::COPY_DST);
    const FORMAT: ShaderStages = ShaderStages::VERTEX;
}


// VertexBufferInfo

pub trait VertexBufferInfo where Self: Pod {
    const ATTRIBUTES: &'static [VertexAttribute];
}

impl<T: VertexBufferInfo> BufferInfo<VertexBufferLayout<'static>> for T {
    const USAGE: BufferUsages = BufferUsages::VERTEX;
    const FORMAT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Self>() as wgpu::BufferAddress,
        step_mode: VertexStepMode::Vertex,
        attributes: &Self::ATTRIBUTES,
    };
}
