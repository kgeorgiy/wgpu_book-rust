use crate::common::{CmdArgs, VertexNC};
use crate::common::colormap::Colormap;
use crate::common::light::TwoSideLightAux;
use crate::common::surface_data::Surface;

#[path = "../common/global_common.rs"]
mod common;


fn main() {
    let colormap = &Colormap::by_name(CmdArgs::next("jet").as_str());
    let (name, triangles) = Surface::read_args_triangles(colormap, true);
    TwoSideLightAux::example(include_str!("shader.wgsl"), triangles.cast::<VertexNC>())
        .run_title(format!("Chapter 09. Surface ({name})").as_str());
}
