use cgmath::{Matrix4, SquareMatrix};

use webgpu_book::{PipelineConfiguration, TextureInfo};

use crate::common::light::{ProtoUniforms, TwoSideLightAux};

pub use self::global_common::*;

#[path = "../common/global_common.rs"]
mod global_common;

#[allow(dead_code)]
pub fn run_example(title: &str, vertices: &[VertexNCT]) -> ! {
    example_models(vertices, [Matrix4::identity()])
        .run_title(title)
}

#[must_use] pub fn example_models<const T: usize>(vertices: &[VertexNCT], models: [Matrix4<f32>; T]) -> PipelineConfiguration {
    let texture_file = CmdArgs::next("whitesquare2");
    let is_two_side = CmdArgs::next("false").parse().expect("true of false");

    let light_aux = TwoSideLightAux::new(is_two_side);

    ProtoUniforms::example_models(
        include_str!("instances.wgsl").to_owned(),
        None,
        light_aux,
        models,
    )
        .into_config()
        .with_vertices(vertices)
        .with_textures([TextureInfo::repeated(format!("examples/ch11/assets/{texture_file}.png"))])
}
