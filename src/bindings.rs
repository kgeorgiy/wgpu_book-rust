use anyhow::{Context, Result};
use image::{io::Reader as ImageReader, RgbaImage};

use crate::{Content, NoContent, UniformsConfiguration};
use crate::buffer::SmartBuffer;
use crate::webgpu::WebGPUDevice;

// Binding

#[derive(Clone, Debug)]
pub(crate) struct Binding<'a> {
    pub(crate) resource: wgpu::BindingResource<'a>,
    pub(crate) visibility: wgpu::ShaderStages,
    pub(crate) ty: wgpu::BindingType,
}


// BindGroup

pub(crate) struct BindGroup {
    pub(crate) group: wgpu::BindGroup,
    pub(crate) layout: wgpu::BindGroupLayout,
}

impl BindGroup {
    pub(crate) fn new(wg: &WebGPUDevice, label: &str, bindings: Vec<Binding>) -> Self {
        let layouts = &bindings.iter().enumerate()
            .map(|(index, binding)| wgpu::BindGroupLayoutEntry {
                binding: index as u32,
                visibility: binding.visibility,
                ty: binding.ty,
                count: None,
            })
            .collect::<Vec<_>>();

        let layout = wg.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(format!("{} Bing Group Layout", label).as_str()),
            entries: layouts,
        });

        let group = wg.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(format!("{} Bing Group", label).as_str(), ),
            layout: &layout,
            entries: &bindings.into_iter().enumerate()
                .map(|(index, binding)| wgpu::BindGroupEntry { binding: index as u32, resource: binding.resource })
                .collect::<Vec<_>>(),
        });

        Self { group, layout }
    }
}


// Uniforms

pub(crate) struct Uniforms {
    pub(crate) content: Box<dyn Content>,
    pub(crate) bindings: BindGroup,
}

impl Uniforms {
    pub(crate) fn new<const UL: usize>(wg: &WebGPUDevice, conf: Option<UniformsConfiguration<UL>>) -> Self {
        conf.map_or_else(|| Self::none(wg), |conf| Self::some(wg, conf))
    }

    fn some<const UL: usize>(wg: &WebGPUDevice, conf: UniformsConfiguration<UL>) -> Self {
        let UniformsConfiguration { buffers, content_factory} = conf;
        let buffers = buffers.into_iter()
            .map(|descriptor| descriptor.create_buffer(wg))
            .collect::<Vec<_>>();

        let bindings = buffers.iter()
            .map(Self::binding)
            .collect::<Vec<_>>();
        let bindings = BindGroup::new(wg, "Uniform", bindings);
        let writers = buffers.into_iter()
            .map(|buffer| buffer.writer(wg.queue.clone()))
            .collect::<Vec<_>>();

        Self { bindings, content: content_factory._unsafe_create(writers) }
    }

    fn none(wg: &WebGPUDevice) -> Self {
        Self {
            content: Box::new(NoContent),
            bindings: BindGroup::new(wg, "Uniform", vec![]),
        }
    }

    fn binding(buffer: &SmartBuffer<wgpu::ShaderStages>) -> Binding {
        Binding {
            resource: buffer.buffer.as_entire_binding().clone(),
            visibility: buffer.format,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
        }
    }
}


// TextureInfo

pub struct TextureInfo {
    pub file: String,
    pub u_mode: wgpu::AddressMode,
    pub v_mode: wgpu::AddressMode,
}

impl TextureInfo {
    pub(crate) fn create_texture(&self, wg: &WebGPUDevice) -> Result<Texture> {
        let img = ImageReader::open(self.file.as_str())
            .context(format!("Texture file '{}' missing", self.file))?.decode()?;
        let image: RgbaImage = img.to_rgba8();

        let size = wgpu::Extent3d {
            width: image.width(),
            height: image.height(),
            depth_or_array_layers: 1,
        };

        let texture = wg.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(format!("Texture {}", self.file).as_str()),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        wg.queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &image,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * image.width()),
                rows_per_image: Some(image.height()),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = wg.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: self.u_mode,
            address_mode_v: self.v_mode,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Texture { _texture: texture, view, sampler, })
    }
}


// Texture

pub struct Texture {
    _texture: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
}

impl Texture {
    fn bindings(&self) -> [Binding; 2] {
        [
            Binding {
                resource: wgpu::BindingResource::TextureView(&self.view),
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
            },
            Binding {
                resource: wgpu::BindingResource::Sampler(&self.sampler),
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            },
        ]
    }
}


// Textures

pub(crate) struct Textures {
    _textures: Vec<Texture>,
    pub(crate) bindings: BindGroup,
}

impl Textures {
    pub(crate) fn new(wg: &WebGPUDevice, texture_infos: &[TextureInfo]) -> Result<Self> {
        let textures: Vec<Texture> = texture_infos.iter()
            .map(|info| info.create_texture(wg))
            .collect::<Result<Vec<_>>>()?;


        let bindings = textures.iter()
            .flat_map(|texture| texture.bindings())
            .collect::<Vec<_>>();

        Ok(Self {
            bindings: BindGroup::new(wg, "Textures", bindings),
            _textures: textures,
        })
    }
}
