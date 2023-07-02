use webgpu_book::TextureInfo;

use crate::common::CmdArgs;
use crate::common::colormap::Colormap;
use crate::common::light::TwoSideLightAux;
use crate::common::surface_data::Surface;

#[path = "../common/global_common.rs"]
mod common;

fn main() {
    let colormap = Colormap::by_name("jet");
    let (name, triangles) = Surface::read_args_triangles(&colormap, false);
    let texture_file = CmdArgs::next("whitesquare2");

    TwoSideLightAux::example(include_str!("shader.wgsl"), triangles)
        .with_textures([TextureInfo::repeated(format!("examples/ch11/assets/{texture_file}.png"))])
        .run_title(format!("Chapter 11. Surface {name}").as_str());
}
