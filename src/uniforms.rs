#![allow(clippy::module_name_repetitions)]

use crate::{BufferWriter, Content, NoContent, SmartBufferDescriptor};
use crate::bindings::{BindGroup, Binding};
use crate::buffer::SmartBuffer;
use crate::webgpu::WebGPUDevice;


// Uniforms Configuration

pub struct UniformsConfiguration {
    buffers: Vec<SmartBufferDescriptor<wgpu::ShaderStages>>,
    content_factory: Box<dyn UnsafeContentFactory>,
}

impl UniformsConfiguration {
    #[must_use] pub fn new<const UL: usize>(
        buffers: [SmartBufferDescriptor<wgpu::ShaderStages>; UL],
        content_factory: Box<dyn ContentFactory<UL>>
    ) -> Option<Self> {
        Some(Self { content_factory: Box::new(content_factory), buffers: buffers.into_iter().collect() })
    }
}


// Content Factories

pub trait ContentFactory<const UL: usize> {
    fn create(self: Box<Self>, uniforms: [BufferWriter; UL]) -> Box<dyn Content>;
}

struct NoContentFactory;

impl ContentFactory<0> for NoContentFactory {
    fn create(self: Box<Self>, _uniforms: [BufferWriter; 0]) -> Box<dyn Content> {
        Box::new(NoContent)
    }
}

trait UnsafeContentFactory {
    fn create(self: Box<Self>, uniforms: Vec<BufferWriter>) -> Box<dyn Content>;
}

impl<const UL: usize> UnsafeContentFactory for Box<dyn ContentFactory<UL>> {
    fn create(self: Box<Self>, uniforms: Vec<BufferWriter>) -> Box<dyn Content> {
        ContentFactory::<UL>::create(*self, uniforms.try_into().expect("valid size"))
    }
}


// Uniforms

pub(crate) struct Uniforms {
    pub(crate) content: Box<dyn Content>,
    pub(crate) bindings: BindGroup,
}

impl Uniforms {
    pub(crate) fn new(wg: &WebGPUDevice, config: Option<UniformsConfiguration>) -> Self {
        config.map_or_else(|| Self::none(wg), |conf| Self::some(wg, conf))
    }

    fn some(wg: &WebGPUDevice, conf: UniformsConfiguration) -> Self {
        let UniformsConfiguration { buffers: descriptions, content_factory} = conf;
        let buffers = descriptions.into_iter()
            .map(|descriptor| descriptor.create_buffer(wg))
            .collect::<Vec<_>>();

        let bindings = buffers.iter().map(Self::bindings).collect::<Vec<_>>();
        let group = BindGroup::new(wg, "Uniform", bindings);
        let writers = buffers.into_iter()
            .map(|buffer| buffer.writer(wg.queue.clone()))
            .collect::<Vec<_>>();

        Self { bindings: group, content: content_factory.create(writers) }
    }

    fn none(wg: &WebGPUDevice) -> Self {
        Self {
            content: Box::new(NoContent),
            bindings: BindGroup::new(wg, "Uniform", vec![]),
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
