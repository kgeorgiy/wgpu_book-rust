use webgpu_book::TextureInfo;

use crate::common::light::TwoSideLightAux;
use crate::common::surface_data::Triangles;

pub use self::global_common::*;

#[path = "../common/global_common.rs"]
mod global_common;

pub fn run_example(title: &str, triangles: Triangles<VertexNCT>) -> ! {
    let texture_file = CmdArgs::next("whitesquare2");

    TwoSideLightAux::example(include_str!("shader.wgsl"), triangles)
        .with_topology(wgpu::PrimitiveTopology::TriangleList)
        .with_textures([TextureInfo::repeated(format!("examples/ch11/assets/{texture_file}.png"))])
        .run_title(title)
}
