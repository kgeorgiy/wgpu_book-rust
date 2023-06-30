use core::ops::{Deref, DerefMut};

use bytemuck::Pod;

use crate::{BufferWriter, Content, NoContent, SmartBufferDescriptor, TypedBufferWriter};
use crate::bindings::{BindGroupVariants, Binding};
use crate::boxed::{BoxedFunc, FuncBox};
use crate::buffer::SmartBuffer;
use crate::webgpu::WebGPUDevice;

// Uniforms Configuration

pub(crate) struct UniformsConfiguration {
    buffers: Vec<SmartBufferDescriptor<wgpu::ShaderStages>>,
    content_factory: UnsafeContentFactory,
    variants: Vec<Vec<usize>>,
}

impl UniformsConfiguration {
    #[must_use]
    pub(crate) fn new<const UL: usize>(
        buffers: [SmartBufferDescriptor<wgpu::ShaderStages>; UL],
        content_factory: ContentFactory<UL>,
        variants: Vec<[usize; UL]>
    ) -> Self {
        for variant in &variants {
            variant.iter().zip(buffers.iter())
                .for_each(|(index, buffer)|
                    assert!(*index < buffer.layout.item_count, "Index out of bounds"));
        }

        Self {
            content_factory: content_factory.before(|bufs: Vec<BufferWriter>| bufs.try_into().expect("valid size")),
            buffers: buffers.into_iter().collect(),
            variants: variants.into_iter()
                .map(move |variant| variant.into_iter().collect())
                .collect()
        }
    }
}


// Content Factories

pub type ContentFactory<const UL: usize> = FuncBox<[BufferWriter; UL], Box<dyn Content>>;

struct NoContentFactory;

impl BoxedFunc<[BufferWriter; 0], Box<dyn Content>> for NoContentFactory {
    fn apply(self: Box<Self>, _uniforms: [BufferWriter; 0]) -> Box<dyn Content> {
        Box::new(NoContent)
    }
}

pub type UnsafeContentFactory = FuncBox<Vec<BufferWriter>, Box<dyn Content>>;

impl<const UL: usize> BoxedFunc<Vec<BufferWriter>, Box<dyn Content>> for ContentFactory<UL> {
    fn apply(self: Box<Self>, uniforms: Vec<BufferWriter>) -> Box<dyn Content> {
        (*self).apply(uniforms.try_into().expect("valid size"))
    }
}


// Uniforms

pub(crate) struct Uniforms {
    pub(crate) content: Box<dyn Content>,
    pub(crate) variants: BindGroupVariants,
}

impl Uniforms {
    pub(crate) fn new(wg: &WebGPUDevice, config: Option<UniformsConfiguration>) -> Self {
        config.map_or_else(|| Self::none(wg), |conf| Self::some(wg, conf))
    }

    fn some(wg: &WebGPUDevice, conf: UniformsConfiguration) -> Self {
        let UniformsConfiguration { buffers: descriptions, content_factory, variants} = conf;
        let buffers = descriptions.into_iter()
            .map(|descriptor| descriptor.create_buffer(wg))
            .collect::<Vec<_>>();


        let bindings: Vec<Binding> = buffers.iter().map(Self::bindings).collect();
        let bg_variants = BindGroupVariants::new(wg, "Uniform", bindings, variants);

        let writers = buffers.into_iter()
            .map(|buffer| buffer.writer(wg.queue.clone()))
            .collect::<Vec<_>>();

        Self {
            variants: bg_variants,
            content: content_factory.apply(writers)
        }
    }

    fn none(wg: &WebGPUDevice) -> Self {
        Self {
            content: Box::new(NoContent),
            variants: BindGroupVariants::new(wg, "Uniform", vec![], vec![]),
        }
    }

    fn bindings(buffer: &SmartBuffer<wgpu::ShaderStages>) -> Binding {
        const TY: wgpu::BindingType = wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        };
        Binding {
            resources: buffer.resources(),
            visibility: buffer.format,
            ty: TY
        }
    }
}


// To

pub trait To<T> {
    fn to(&self) -> T;
}

impl<T: Clone> To<T> for T {
    fn to(&self) -> T {
        self.clone()
    }
}


// UniformArray

pub struct UniformArray<B: Pod, const L: usize>([B; L]);

impl<B: Pod, const L: usize> UniformArray<B, L> {
    pub fn as_instances(&self) -> [[B; L]; 1] {
        [self.0]
    }

    pub fn as_bindings(&self) -> &[B; L] {
        &self.0
    }
}

impl<T, B, const L: usize> To<UniformArray<B, L>> for [T; L] where B: Pod, T: To<B> + Clone {
    fn to(&self) -> UniformArray<B, L> {
        UniformArray(self.clone().map(|model| model.to()))
    }
}



// Uniform and UniformState

pub struct Uniform<T> {
    state: T,
    write: Box<dyn Fn(&T)>,
}

impl<T> Uniform<T> {
    pub fn new(state: T, write: Box<dyn Fn(&T)>) -> Self {
        Self { state, write }
    }

    pub(crate) fn value<B>(state: T, buffer: BufferWriter) -> Self where B: 'static + Pod, T: To<B> {
        let typed_buffer: TypedBufferWriter<B> = buffer.to_typed();
        Self::new(state, Box::new(move |value| typed_buffer.write_slice(&[value.to()])))
    }
}

impl<T, const L: usize> Uniform<[T; L]> {
    pub(crate) fn instance_array<B>(state: [T; L], buffer: BufferWriter)
        -> Self where B: Pod, T: To<B> + Clone
    {
        let tb: TypedBufferWriter<[B; L]> = buffer.to_typed();
        Self::new(
            state,
            Box::new(move |values| tb.write_slice(&[values.clone().map(|v| v.to())]))
        )
    }

    pub(crate) fn binding_array<B>(state: [T; L], buffer: BufferWriter)
        -> Self where B: Pod, T: To<B>
    {
        let tb: TypedBufferWriter<B> = buffer.to_typed();
        Self::new(
            state,
            Box::new(move |values| tb.write_slice(&values.iter().map(To::to).collect::<Vec<B>>()))
        )
    }
}

impl<T> Uniform<T> {
    fn write(&self) {
        (self.write)(&self.state);
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
