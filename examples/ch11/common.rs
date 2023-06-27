use webgpu_book::{RenderConfiguration, TextureInfo};

use crate::common::light::{ProtoUniforms, TwoSideLightAux};

pub use self::global_common::*;

#[path = "../common/global_common.rs"]
mod global_common;

pub fn run_example(title: &str, vertices: &[VertexNCT]) -> ! {
    let texture_file = CmdArgs::next("whitesquare2");
    let is_two_side = CmdArgs::next("false").parse().expect("true of false");

    let configuration = RenderConfiguration {
        textures: vec![TextureInfo {
            file: format!("examples/ch11/assets/{texture_file}.png"),
            u_mode: wgpu::AddressMode::Repeat,
            v_mode: wgpu::AddressMode::Repeat,
        }],
        ..ProtoUniforms::example_aux(
            include_str!("shader.wgsl").to_owned(),
            None,
            TwoSideLightAux::new(is_two_side)
        ).config(
            include_str!("shader.wgsl"),
            wgpu::PrimitiveTopology::TriangleList,
            vertices,
        )
    };
    configuration.run_title(title);
}