use webgpu_book::TextureInfo;

use crate::common::light::TwoSideLight;
use crate::common::surface_data::Triangles;

pub use self::global_common::*;

#[path = "../common/global_common.rs"]
mod global_common;

pub fn run_example(title: &str, triangles: Triangles<VertexNT>) -> ! {
    let texture_file = CmdArgs::next("earth");

    TwoSideLight::example(include_str!("shader.wgsl"), triangles)
        .with_textures([TextureInfo::repeated(format!("examples/ch10/assets/{texture_file}.png"))])
        .run_title(title)
}

