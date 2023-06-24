use std::{any::TypeId, marker::PhantomData, mem::size_of, rc::Rc};

use bytemuck::{cast_slice, Pod};
use wgpu::{Buffer, BufferUsages, IndexFormat, Queue, ShaderStages, VertexAttribute, VertexBufferLayout, VertexStepMode};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

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
}

// SmartBufferDescriptor

pub struct SmartBufferDescriptor<'a, F: Clone> {
    descriptor: BufferInitDescriptor<'a>,
    layout: BufferLayout,
    format: F,
}

impl<'a, F: Clone> SmartBufferDescriptor<'a, F> {
    pub fn new<T: Pod>(label: &'a str, items: &'a [T], usage: BufferUsages, format: F) -> Self {
        Self {
            descriptor: BufferInitDescriptor {
                label: Some(label),
                contents: cast_slice(items),
                usage,
            },
            layout: BufferLayout {
                len: items.len(),
                type_id: TypeId::of::<T>(),
            },
            format,
        }
    }

    pub(crate) fn create_buffer(&self, device: &WebGPUDevice) -> SmartBuffer<F> {
        let buffer = Rc::new(device.device.create_buffer_init(&self.descriptor));
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

    fn buffer<'a>(label: &'a str, items: &'a [Self]) -> SmartBufferDescriptor<'a, F> {
        Self::buffer_format(label, items, Self::FORMAT.clone())
    }

    fn buffer_format<'a>(label: &'a str, items: &'a [Self], format: F) -> SmartBufferDescriptor<'a, F> {
        SmartBufferDescriptor::new(label, items, Self::USAGE, format)
    }
}

impl<T: Pod> BufferInfo<IndexFormat> for T {
    const USAGE: BufferUsages = BufferUsages::INDEX;
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
