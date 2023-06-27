#![allow(dead_code)]

use core::cell::RefCell;
use core::ops::{Deref, DerefMut};

use bytemuck::Pod;

pub use vertex::*;
use webgpu_book::{BufferWriter, TypedBufferWriter};

pub mod colormap;
pub mod vertex_data;
pub mod functions;
pub mod light;
pub mod surface_data;
pub mod mvp;
mod vertex;

pub(crate) struct CmdArgs;

thread_local!(static ARGS: RefCell<Vec<String>> = RefCell::new(std::env::args().skip(1).rev().collect()));

impl CmdArgs {
    pub(crate) fn next(default: &str) -> String {
        ARGS.with(|cell| cell.borrow_mut().pop().unwrap_or(default.to_owned()))
    }

    pub(crate) fn next_known(known: &[&str]) -> String {
        let value = Self::next(known.first().expect("at least one known variant"));
        assert!(known.iter().any(|k| value == *k), "Unknown argument '{value}', expected one of {known:?}");
        value
    }

    pub(crate) fn is(expected: &str) -> bool {
        ARGS.with(|cell| {
            let mut args = cell.borrow_mut();
            let result = args.starts_with(&[expected.to_owned()]);
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
        self.buffer.write(self.state.to());
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
