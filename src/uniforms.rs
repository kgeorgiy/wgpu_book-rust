use core::cell::RefCell;
use core::ops::{Deref, DerefMut};
use std::rc::Rc;

use bytemuck::Pod;

use crate::{BufferInfo, BufferWriter, SmartBufferDescriptor};
use crate::bindings::{BindGroupVariants, Binding};
use crate::buffer::SmartBuffer;
use crate::webgpu::WebGPUDevice;

//
// Uniforms

pub(crate) struct Uniforms {
    pub(crate) variants: BindGroupVariants,
    pub(crate) instances: usize,
}

impl Uniforms {
    const TY: wgpu::BindingType = wgpu::BindingType::Buffer {
        ty: wgpu::BufferBindingType::Uniform,
        has_dynamic_offset: false,
        min_binding_size: None,
    };

    pub fn new(conf: UniformsConfiguration, wg: &WebGPUDevice) -> Self {
        let UniformsConfiguration {uniforms, variants, instances } = conf;
        let buffers = uniforms.into_iter()
            .map(|uniform| uniform.resolve(wg))
            .collect::<Vec<_>>();
        let bindings = buffers.iter()
            .map(|buffer| Binding {
                resources: buffer.resources(),
                visibility: buffer.format,
                ty: Self::TY,
            })
            .collect();
        Self {
            variants: BindGroupVariants::new(wg, "Uniforms", bindings, variants),
            instances,
        }
    }
}

//
// To

pub trait To<T> {
    fn to(&self) -> T;
}

impl<T: Clone> To<T> for T {
    fn to(&self) -> T {
        self.clone()
    }
}

//
// UniformConfig

pub(crate) struct UniformConfig {
    pub(crate) buffer: SmartBufferDescriptor<wgpu::ShaderStages>,
    pub(crate) writer: Rc<RefCell<Option<BufferWriter>>>,
}

impl UniformConfig {
    pub(crate) fn resolve(self, wg: &WebGPUDevice) -> SmartBuffer<wgpu::ShaderStages> {
        let buffer = self.buffer.create_buffer(wg);
        *(self.writer.borrow_mut()) = Some(buffer.writer(wg.queue.clone()));
        buffer
    }
}

//
// UniformsConfiguration and UniformAdd

#[must_use]
pub struct UniformsConfiguration {
    uniforms: Vec<UniformConfig>,
    variants: Vec<Vec<usize>>,
    instances: usize,
}

impl UniformsConfiguration {
    pub fn add<T>(&mut self, label: &str, value: T, stages: wgpu::ShaderStages) -> UniformAdd<T> {
        UniformAdd {
            uniforms: &mut self.uniforms,
            label: label.to_owned(),
            value,
            stages,
        }
    }

    pub fn instances(&mut self, instances: usize) -> &mut Self {
        self.instances = instances;
        self
    }

    pub fn variants(&mut self, variants: Vec<Vec<usize>>) -> &mut Self {
        self.variants = variants;
        self
    }
}

impl Default for UniformsConfiguration {
    fn default() -> Self {
        Self { uniforms: vec![], variants: vec![vec![]], instances: 1 }
    }
}

pub struct UniformAdd<'a, T> {
    uniforms: &'a mut Vec<UniformConfig>,
    label: String,
    value: T,
    stages: wgpu::ShaderStages,
}

impl<'a, T> UniformAdd<'a, T> {
    pub fn value<B>(self) -> Uniform<T> where T: To<B>, B: Pod {
        let cast: fn(&T) -> Vec<B> = Self::cast_value;
        let write: fn(&T, &BufferWriter) = |value, buffer| buffer.write_slice(&Self::cast_value(value));
        self.build(cast, write)
    }

    fn build<B>(self, cast: fn(&T) -> Vec<B>, write: fn(&T, &BufferWriter)) -> Uniform<T> where B: Pod {
        let buffer = B::buffer_format(self.label.as_str(), &cast(&self.value), self.stages);
        let uniform = Uniform::new(self.value, write);
        self.uniforms.push(UniformConfig { buffer, writer: uniform.buffer.clone() });
        uniform
    }

    fn cast_value<B>(value: &T) -> Vec<B> where T: To<B>, B: Pod {
        vec![value.to()]
    }
}

impl<'a, T: Clone, const L: usize> UniformAdd<'a, [T; L]> {
    pub fn instance_array<B>(self) -> Uniform<[T; L]> where T: To<B>, B: Pod {
        let cast = Self::cast_ia;
        let write = |vs: &[T; L], b: &BufferWriter| b.write_slice(&Self::cast_ia(vs));
        self.build(cast, write)
    }

    fn cast_ia<B>(values: &[T; L]) -> Vec<[B; L]> where T: To<B>, B: Pod  {
        vec![values.clone().map(|v| v.to())]
    }

    pub fn bindings_array<B>(self) -> Uniform<[T; L]> where T: To<B>, B: Pod {
        let cast = Self::cast_ba;
        let write = |values: &[T; L], buffer: &BufferWriter| buffer.write_slice(&Self::cast_ba(values));
        self.build::<B>(cast, write)
    }

    fn cast_ba<B>(values: &[T; L]) -> Vec<B> where T: To<B>, B: Pod {
        values.iter().map(To::to).collect::<Vec<B>>()
    }
}

//
// Uniform and UniformMut

pub struct Uniform<T> {
    state: T,
    write: fn(&T, &BufferWriter),
    pub(crate) buffer: Rc<RefCell<Option<BufferWriter>>>,
}

impl<T> Uniform<T> {
    fn new(state: T, write: fn(&T, &BufferWriter)) -> Self {
        Self { state, write, buffer: Rc::new(RefCell::new(None)) }
    }
}

impl<T> Uniform<T> {
    fn write(&self) {
        if let Some(buffer) = (*self.buffer.borrow()).as_ref() {
            (self.write)(&self.state, buffer);
        }
    }

    pub fn as_mut(&mut self) -> UniformMut<T> {
        UniformMut { uniform: self }
    }
}

impl<T> Deref for Uniform<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.state
    }
}

pub struct UniformMut<'a, T> {
    uniform: &'a mut Uniform<T>,
}

impl<T> Drop for UniformMut<'_, T> {
    fn drop(&mut self) {
        self.uniform.write();
    }
}

impl<T> Deref for UniformMut<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.uniform.state
    }
}

impl<T> DerefMut for UniformMut<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.uniform.state
    }
}
