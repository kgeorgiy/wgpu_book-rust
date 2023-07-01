use cgmath::point3;

use webgpu_book::RenderPassConfiguration;

use crate::common::colormap::Colormap;
use crate::common::light::TwoSideLightAux;
use crate::common::surface_data::Surface;
use crate::common::VertexNCT;

mod common;
#[path = "../ch07/ex_torus.rs"]
mod ex_torus;

fn main() -> ! {
    let surface = Surface::read_args_surface();
    let colormap = &Colormap::by_name("jet");

    let edges = common::edges_pipeline(surface.edges(point3(1.0, 1.0, 1.0)).cast());

    let triangles = TwoSideLightAux::example(
        include_str!("../ch09/shader.wgsl"),
        surface.triangles(colormap, false).cast::<VertexNCT>()
    );

    let axes = common::edges_pipeline(surface.axes(2.5));

    RenderPassConfiguration::new(vec![edges, triangles, axes])
        .run_title(format!("Chapter 12. Surfaces ({})", surface.name()).as_str())
}
