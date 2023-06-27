use std::{any::TypeId, marker::PhantomData, mem::size_of, rc::Rc};

use bytemuck::{cast_slice, Pod};
use wgpu::{Buffer, BufferUsages, IndexFormat, Queue, ShaderStages, VertexAttribute, VertexBufferLayout, VertexStepMode};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::webgpu::WebGPUDevice;

// BufferFormat

#[derive(Clone, Debug)]
pub(crate) struct BufferLayout {
    type_id: TypeId,
    len: usize,
}


// SmartBuffer

pub(crate) struct SmartBuffer<F> {
    pub(crate) buffer: Buffer,
    pub(crate) format: F,
    layout: BufferLayout,
}

impl<F> SmartBuffer<F> {
    pub(crate) fn writer(self, queue: Rc<Queue>) -> BufferWriter {
        BufferWriter {
            buffer: self.buffer,
            layout: self.layout,
            queue: queue.clone(),
        }
    }
}


// BufferWriter

#[derive(Debug)]
pub struct BufferWriter {
    queue: Rc<Queue>,
    pub(crate) buffer: Buffer,
    layout: BufferLayout,
}

impl BufferWriter {
    pub fn to_typed<T: 'static>(self) -> TypedBufferWriter<T> {
        assert_eq!(self.layout.type_id, TypeId::of::<T>());
        TypedBufferWriter { untyped: self, phantom: Default::default() }
    }

    fn write_slice<T: Pod + 'static>(&self, slice: &[T]) {
        assert_eq!(self.layout.len, slice.len());
        self.queue.write_buffer(&self.buffer, 0, cast_slice(slice));
    }
}

// TypedBufferWriter

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

pub struct SmartBufferDescriptor<F> {
    label: String,
    contents: Vec<u8>,
    usage: BufferUsages,
    layout: BufferLayout,
    format: F,
}

impl<'a, F> SmartBufferDescriptor<F> {
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

    pub(crate) fn create_buffer(self, wg: &WebGPUDevice) -> SmartBuffer<F> {
        let buffer = wg.device.create_buffer_init(&BufferInitDescriptor {
            label: Some(self.label.as_str()),
            contents: &self.contents,
            usage: self.usage,
        });
        SmartBuffer { buffer, format: self.format, layout: self.layout }
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
