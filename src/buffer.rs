use std::marker::PhantomData;
use std::mem::size_of;
use std::rc::Rc;

use bytemuck::{cast_slice, Pod};
use wgpu::{Buffer, BufferUsages, Device, IndexFormat, Queue, VertexAttribute, VertexBufferLayout, VertexStepMode};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use crate::webgpu::WebGPUDevice;

// UntypedBuffer

#[derive(Clone)]
pub struct UntypedBuffer<F: Clone> {
    pub(crate) buffer: Rc<Buffer>,
    pub(crate) format: F,
}


// BufferWriter

#[derive(Clone)]
pub struct BufferWriter<T> {
    queue: Rc<Queue>,
    buffer: Rc<Buffer>,
    len: usize,
    phantom: PhantomData<T>,
}

impl<T: Pod> BufferWriter<T> {
    pub fn write_slice(&self, slice: &[T]) {
        assert_eq!(self.len, slice.len());
        self.queue.write_buffer(&self.buffer, 0, cast_slice(slice));
    }
}


// TypedBuffer

#[derive(Clone)]
pub(crate) struct TypedBuffer<T, F: Clone> {
    pub(crate) buffer: UntypedBuffer<F>,
    pub(crate) writer: BufferWriter<T>,
}


// UntypedBufferDescriptor

pub trait UntypedBufferDescriptor<F: Clone> {
    fn create_buffer(&self, device: &Device) -> UntypedBuffer<F>;
}


// TypedBufferDescriptor

pub struct TypedBufferDescriptor<'a, T, F: Clone> {
    pub(crate) descriptor: BufferInitDescriptor<'a>,
    pub(crate) format: F,
    pub(crate) len: usize,
    phantom: PhantomData<T>,
}

impl<'a, T: Pod, F: Clone> TypedBufferDescriptor<'a, T, F> {
    pub fn new(label: &'a str, items: &'a [T], usage: BufferUsages, format: F) -> Self {
        TypedBufferDescriptor {
            descriptor: BufferInitDescriptor {
                label: Some(label),
                contents: cast_slice(items),
                usage,
            },
            len: items.len(),
            format,
            phantom: Default::default(),
        }
    }

    pub(crate) fn create_buffer(&self, device: &WebGPUDevice) -> TypedBuffer<T, F> {
        let buffer = device.create_untyped_buffer(self);
        TypedBuffer {
            writer: BufferWriter {
                queue: device.queue.clone(),
                buffer: buffer.buffer.clone(),
                len: self.len,
                phantom: Default::default(),
            },
            buffer,
        }
    }
}

impl<'a, T: Pod, F: Clone> UntypedBufferDescriptor<F> for TypedBufferDescriptor<'a, T, F> {
    fn create_buffer(&self, device: &Device) -> UntypedBuffer<F> {
        UntypedBuffer {
            buffer: Rc::new(device.create_buffer_init(&self.descriptor)),
            format: self.format.clone(),
        }
    }
}


// BufferInfo

pub trait BufferInfo<F: Clone + 'static> where Self: Pod {
    const USAGE: BufferUsages;
    const FORMAT: F;

    fn buffer<'a>(label: &'a str, items: &'a [Self]) -> Box<dyn UntypedBufferDescriptor<F> + 'a> {
        Box::new(Self::typed_buffer(label, items))
    }

    fn typed_buffer<'a>(label: &'a str, items: &'a [Self]) -> TypedBufferDescriptor<'a, Self, F> {
        TypedBufferDescriptor::new(label, items, Self::USAGE, Self::FORMAT.clone())
    }
}

impl BufferInfo<IndexFormat> for u16 {
    const USAGE: BufferUsages = BufferUsages::INDEX;
    const FORMAT: IndexFormat = IndexFormat::Uint16;
}


// VertexBufferInfo

pub trait VertexBufferInfo where Self: Pod {
    const ATTRIBUTES: &'static [VertexAttribute];
}

impl <T: VertexBufferInfo> BufferInfo<VertexBufferLayout<'static>> for T {
    const USAGE: BufferUsages = BufferUsages::VERTEX;
    const FORMAT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Self>() as wgpu::BufferAddress,
        step_mode: VertexStepMode::Vertex,
        attributes: &Self::ATTRIBUTES,
    };
}
