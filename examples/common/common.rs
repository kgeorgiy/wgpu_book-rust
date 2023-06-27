#![allow(dead_code)]

use ::std::cell::RefCell;
use std::ops::{Deref, DerefMut};

use bytemuck::Pod;

use ::webgpu_book::{BufferInfo, IndexBufferInfo, RenderConfiguration, VertexBufferInfo};
pub use mvp::*;
pub use vertex::*;
use webgpu_book::{BufferWriter, TypedBufferWriter};

pub mod colormap;
pub mod vertex_data;
pub mod functions;
pub mod light;
pub mod surface_data;
mod vertex;
mod mvp;

pub(crate) struct Config;

impl Config {
    pub fn with_vertices<V, I, const UL: usize>(shader_source: &str, vertices: &[V], indices: Option<&[I]>)
        -> RenderConfiguration<UL> where V: VertexBufferInfo, I: IndexBufferInfo
    {
        RenderConfiguration {
            vertices: indices.map_or(vertices.len(), |idx| idx.len()),
            vertex_buffers: vec![V::buffer("Vertices", &vertices)],
            index_buffer: indices.map(|idx| I::buffer("Indices", idx)),
            ..Self::with_shader(shader_source)
        }
    }

    pub(crate) fn with_shader<const UL: usize>(shader_source: &str) -> RenderConfiguration<UL> {
        RenderConfiguration {
            shader_source: shader_source.to_string(),
            ..RenderConfiguration::default()
        }
    }
}

pub(crate) struct CmdArgs;

thread_local!(static ARGS: RefCell<Vec<String>> = RefCell::new(std::env::args().skip(1).rev().collect()));

impl CmdArgs {
    pub(crate) fn next(default: &str) -> String {
        ARGS.with(|cell| cell.borrow_mut().pop().unwrap_or(default.to_string()))
    }

    pub(crate) fn next_known(known: &[&str]) -> String {
        let value = Self::next(known[0]);
        assert!(known.iter().any(|k| value == k.to_string()), "Unknown argument '{}', expected one of {:?}", value, known);
        value
    }

    pub(crate) fn is(expected: &str) -> bool {
        ARGS.with(|cell| {
            let mut args = cell.borrow_mut();
            let result = args.starts_with(&[expected.to_string()]);
            if result {
                args.pop();
            }
            result
        })
    }
}


pub trait To<T> {
    fn to(&self) -> T;
}

impl<T: Clone> To<T> for T {
    fn to(&self) -> T {
        self.clone()
    }
}

// Uniform and UniformState

pub struct Uniform<T, B> where B: Pod, T: To<B> {
    state: T,
    buffer: TypedBufferWriter<B>
}

impl<B: 'static, T> Uniform<T, B> where B: Pod, T: To<B> {
    pub(crate) fn new(state: T, buffer: BufferWriter) -> Self {
        Self { state, buffer: buffer.to_typed() }
    }
}

impl<B: Pod, T> Uniform<T, B> where T: To<B> {
    fn write(&self) {
        self.buffer.write(self.state.to())
    }
}

impl<T, B> Deref for Uniform<T, B> where B: Pod, T: To<B> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.state
    }
}

impl<T, B> Uniform<T, B> where B: Pod, T: To<B> {
    pub(crate) fn as_mut(&mut self) -> UniformMut<T, B> {
        UniformMut { uniform: self }
    }
}

pub struct UniformMut<'a, T, B> where B: Pod, T: To<B> {
    uniform: &'a mut Uniform<T, B>,
}

impl<T, B> Drop for UniformMut<'_, T, B> where B: Pod, T: To<B> {
    fn drop(&mut self) {
        self.uniform.write();
    }
}

impl<T, B> Deref for UniformMut<'_, T, B> where B: Pod, T: To<B> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.uniform.state
    }
}

impl<T, B> DerefMut for UniformMut<'_, T, B> where B: Pod, T: To<B> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.uniform.state
    }
}
