#![allow(clippy::module_name_repetitions)]

use core::{any::TypeId, marker::PhantomData, mem::size_of};
use std::rc::Rc;

use bytemuck::{cast_slice, Pod};
use wgpu::{BindingResource, Buffer, BufferAddress, BufferBinding, BufferSize, BufferUsages, IndexFormat, Queue, ShaderStages, VertexAttribute, VertexBufferLayout, VertexStepMode};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::webgpu::WebGPUDevice;

// BufferFormat

#[derive(Clone, Debug)]
pub(crate) struct BufferLayout {
    type_id: TypeId,
    item_count: usize,
    item_size: usize,
    item_alignment: usize,
}


// SmartBuffer

pub(crate) struct SmartBuffer<F> {
    pub(crate) buffer: Buffer,
    pub(crate) format: F,
    pub(crate) layout: BufferLayout,
}

impl<F> SmartBuffer<F> {
    pub(crate) fn writer(self, queue: Rc<Queue>) -> BufferWriter {
        BufferWriter {
            buffer: self.buffer,
            layout: self.layout,
            queue,
        }
    }

    pub(crate) fn resources(&self) -> Vec<BindingResource> {
        (0..self.layout.item_count)
            .map(|index| BindingResource::Buffer(BufferBinding {
                buffer: &self.buffer,
                offset: (index * self.layout.item_alignment) as BufferAddress,
                size: BufferSize::new(self.layout.item_size as u64),
            }))
            .collect()
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
    /// Converts this buffer to typed one
    ///
    /// # Panics
    /// If type of buffer does not equal to requested type
    pub fn to_typed<T: 'static>(self) -> TypedBufferWriter<T> {
        assert_eq!(self.layout.type_id, TypeId::of::<T>(), "Invalid buffer type");
        TypedBufferWriter { untyped: self, phantom: PhantomData::default() }
    }

    fn write_slice<T: Pod + 'static>(&self, slice: &[T]) {
        assert_eq!(self.layout.item_count, slice.len(), "Invalid slice length");
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
    pub fn new<T: Pod>(label: String, items: &'a [T], usage: BufferUsages, format: F, alignment: usize) -> Self {
        Self {
            label,
            contents: cast_slice(items).to_vec(),
            usage,
            layout: BufferLayout {
                item_count: items.len(),
                type_id: TypeId::of::<T>(),
                item_size: size_of::<T>(),
                item_alignment: alignment
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
    const ALIGNMENT: usize = 1;

    fn buffer(label: &str, items: &[Self]) -> SmartBufferDescriptor<F> {
        Self::buffer_format(label, items, Self::FORMAT)
    }

    fn buffer_format(label: &str, items: &[Self], format: F) -> SmartBufferDescriptor<F> {
        SmartBufferDescriptor::new(label.to_owned(), items, Self::USAGE, format, Self::ALIGNMENT)
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
        array_stride: size_of::<Self>() as BufferAddress,
        step_mode: VertexStepMode::Vertex,
        attributes: Self::ATTRIBUTES,
    };
}
