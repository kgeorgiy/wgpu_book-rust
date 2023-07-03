use cgmath::point3;
use webgpu_book::{RenderConfiguration, RenderPassConfiguration};

use crate::common::{edges_pipeline, VertexNC};
use crate::common::colormap::Colormap;
use crate::common::light::TwoSideLight;
use crate::common::surface_data::Surface;

mod common;
#[path = "../ch07/ex_torus.rs"]
mod ex_torus;

fn main() -> ! {
    // let surface = Surface::read_args_surface();
    let surface = Surface::by_name("sphere");
    let colormap = &Colormap::by_name("jet");

    let edges = edges_pipeline(surface.edges(point3(1.0, 1.0, 1.0)).cast());

    let faces = TwoSideLight::example(
        include_str!("../ch09/shader.wgsl"),
        surface.triangles(colormap, false).cast::<VertexNC>(),
    );

    let axes = edges_pipeline(surface.axes(2.5));

    RenderConfiguration::new(vec![
        RenderPassConfiguration::new(vec![faces]),
        RenderPassConfiguration::new(vec![edges, axes])
            .with_load(wgpu::LoadOp::Load),
    ])
        .run_title(format!("Chapter 12. Two-pass rendering ({})", surface.name()).as_str())
}
