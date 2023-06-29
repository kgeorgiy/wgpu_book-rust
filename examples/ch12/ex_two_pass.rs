use cgmath::point3;
use webgpu_book::{RenderConfiguration, RenderPassConfiguration};

use crate::common::{VertexC, VertexNC};
use crate::common::colormap::Colormap;
use crate::common::light::TwoSideLightAux;
use crate::common::surface_data::Surface;

mod common;
#[path = "../ch07/ex_torus.rs"]
mod ex_torus;

fn main() -> ! {
    // let surface = Surface::read_args_surface();
    let surface = Surface::by_name("sphere");
    let colormap = &Colormap::by_name("jet");

    let wireframe_vertices: Vec<VertexC> = surface.wireframe_vertices(point3(1.0, 1.0, 1.0));
    let mesh = TwoSideLightAux::example(include_str!("mesh.wgsl"), wireframe_vertices)
        .with_topology(wgpu::PrimitiveTopology::LineList);

    let surface_vertices: Vec<VertexNC> = surface.surface_vertices(colormap, false);
    let faces = TwoSideLightAux::example(include_str!("../ch09/shader.wgsl"), surface_vertices);
    
    let axes_vertices: Vec<VertexC> = surface.axes_vertices();
    let axes = TwoSideLightAux::example(include_str!("mesh.wgsl"), axes_vertices)
        .with_topology(wgpu::PrimitiveTopology::LineList);

    RenderConfiguration::new(vec![
        RenderPassConfiguration::new(vec![faces]),
        RenderPassConfiguration::new(vec![mesh, axes])
            .with_load(wgpu::LoadOp::Load),
    ])
        .run_title(format!("Chapter 12. Two-pass rendering ({})", surface.name).as_str())
}
