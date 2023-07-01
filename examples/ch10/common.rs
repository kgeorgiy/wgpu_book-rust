use webgpu_book::TextureInfo;

use crate::common::light::TwoSideLightAux;
use crate::common::surface_data::Triangles;

pub use self::global_common::*;

#[path = "../common/global_common.rs"]
mod global_common;

pub fn run_example(title: &str, triangles: Triangles<VertexNT>) -> ! {
    let texture_file = CmdArgs::next("earth");

    TwoSideLightAux::example(include_str!("shader.wgsl"), triangles)
        .with_textures([TextureInfo::repeated(format!("examples/ch10/assets/{texture_file}.png"))])
        .with_cull_mode(Some(wgpu::Face::Back))
        .run_title(title)
}

