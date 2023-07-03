use bytemuck::{Pod, Zeroable};
use cgmath::point3;
use webgpu_book::{PipelineConfiguration, UniformInfo, VertexBufferInfo};

use crate::common::light::LightExamples;
use crate::common::surface_data::Triangles;

pub use self::global_common::*;

#[path = "../common/global_common.rs"]
mod global_common;

// LightAux

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ColorLight {
    color: [f32; 4],
}

impl UniformInfo for ColorLight {
    const STRUCT_NAME: &'static str = "ColorLight";
    const BINDING_NAME: &'static str = "color_light_u";
    const ATTRIBUTES: &'static [(&'static str, &'static str)] = &[
        ("color", "vec4<f32>"),
    ];
}

impl ColorLight {
    #[allow(dead_code)]
    pub fn example<V: VertexBufferInfo + Into<VertexN>>(triangles: Triangles<V>) -> PipelineConfiguration {
        let aux = ColorLight { color: point3(1.0, 0.0, 0.0).to_homogeneous().into() };
        PipelineConfiguration::new(include_str!("shader.wgsl"))
            .with(LightExamples::aux(aux))
            .with_cull_mode(None)
            .with(LightExamples::read_args_wireframe(triangles))
    }
}
