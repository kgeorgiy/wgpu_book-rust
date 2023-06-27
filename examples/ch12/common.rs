use cgmath::{Matrix4, SquareMatrix};

use webgpu_book::{RenderConfiguration, TextureInfo};

use crate::common::light::{ProtoUniforms, TwoSideLightAux};

pub use self::global_common::*;

#[path = "../common/global_common.rs"]
mod global_common;

#[allow(dead_code)]
pub fn run_example(title: &str, vertices: &[VertexNCT]) -> ! {
    run_example_models(title, vertices, [Matrix4::identity()])
}

pub fn run_example_models<const T: usize>(title: &str, vertices: &[VertexNCT], models: [Matrix4<f32>; T]) -> ! {
    let texture_file = CmdArgs::next("whitesquare2");
    let is_two_side = CmdArgs::next("false").parse().expect("true of false");

    let light_aux = TwoSideLightAux::new(is_two_side);

    RenderConfiguration {
        textures: vec![TextureInfo {
            file: format!("examples/ch11/assets/{texture_file}.png"),
            u_mode: wgpu::AddressMode::Repeat,
            v_mode: wgpu::AddressMode::Repeat,
        }],
        ..ProtoUniforms::example_models(
            String::new(),
            None,
            light_aux,
            models,
        ).config(
            include_str!("instances.wgsl"),
            wgpu::PrimitiveTopology::TriangleList,
            vertices,
        )
    }.run_title(title)
}
