use anyhow::{Context, Result};
use image::{io::Reader as ImageReader, RgbaImage};

use crate::usize_as_u32;
use crate::webgpu::WebGPUDevice;

//
// Binding

#[derive(Clone, Debug)]
pub(crate) struct Binding<'a> {
    pub(crate) visibility: wgpu::ShaderStages,
    pub(crate) ty: wgpu::BindingType,
    pub(crate) resources: Vec<wgpu::BindingResource<'a>>,
}

//
// BindGroupVariants

pub(crate) struct BindGroupVariants {
    pub(crate) layout: wgpu::BindGroupLayout,
    pub(crate) groups: Vec<wgpu::BindGroup>,
}

impl BindGroupVariants {
    #[allow(clippy::needless_pass_by_value)]
    pub(crate) fn new(
        wg: &WebGPUDevice,
        label: &str,
        bindings: Vec<Binding>,
        variants: Vec<Vec<usize>>
    ) -> Self {
        let layouts = &bindings.iter().enumerate()
            .map(|(index, binding)| wgpu::BindGroupLayoutEntry {
                binding: usize_as_u32(index),
                visibility: binding.visibility,
                ty: binding.ty,
                count: None,
            })
            .collect::<Vec<_>>();

        let layout = wg.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(format!("{label} Bing Group Layout").as_str()),
            entries: layouts,
        });

        #[allow(clippy::indexing_slicing)]
        let groups: Vec<wgpu::BindGroup> = variants.into_iter().enumerate()
            .map(|(no, variant)| wg.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(format!("{label} Bing Group, variant {no}").as_str()),
                layout: &layout,
                entries: &bindings.iter()
                    .enumerate()
                    .map(|(index, binding)| wgpu::BindGroupEntry {
                        binding: usize_as_u32(index),
                        resource: binding.resources[*variant.get(index).unwrap_or(&0)].clone()
                    })
                    .collect::<Vec<_>>(),
            })).collect();

        Self { layout, groups }
    }
}

//
// TextureInfo

pub struct TextureInfo {
    pub file: String,
    pub u_mode: wgpu::AddressMode,
    pub v_mode: wgpu::AddressMode,
}

impl TextureInfo {
    #[must_use] pub fn repeated(file: String) -> Self {
        TextureInfo {
            file,
            u_mode: wgpu::AddressMode::Repeat,
            v_mode: wgpu::AddressMode::Repeat,
        }
    }

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

//
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
                resources: vec![wgpu::BindingResource::TextureView(&self.view)],
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
            },
            Binding {
                resources: vec![wgpu::BindingResource::Sampler(&self.sampler)],
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            },
        ]
    }
}

//
// Textures

pub(crate) struct Textures {
    _textures: Vec<Texture>,
    pub(crate) variants: BindGroupVariants,
}

impl Textures {
    pub(crate) fn new(wg: &WebGPUDevice, texture_infos: &[TextureInfo]) -> Result<Self> {
        let textures: Vec<Texture> = texture_infos.iter()
            .map(|info| info.create_texture(wg))
            .collect::<Result<Vec<_>>>()?;

        let bindings = textures.iter().flat_map(Texture::bindings).collect::<Vec<_>>();

        Ok(Self {
            variants: BindGroupVariants::new(wg, "Textures", bindings, vec![vec![0, 0]]),
            _textures: textures,
        })
    }
}
