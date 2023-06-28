use cgmath::point3;
use webgpu_book::RenderPassConfiguration;

use crate::common::{VertexC, VertexNC};
use crate::common::colormap::Colormap;
use crate::common::light::TwoSideLightAux;
use crate::common::surface_data::Surface;

mod common;
#[path = "../ch07/ex_torus.rs"]
mod ex_torus;

fn main() -> ! {
    let surface = Surface::read_args_surface();
    let colormap = &Colormap::by_name("jet");

    let wireframe_vertices: Vec<VertexC> = surface.wireframe_vertices(point3(1.0, 1.0, 1.0));
    let mesh = TwoSideLightAux::example(include_str!("mesh.wgsl"), &wireframe_vertices)
        .with_topology(wgpu::PrimitiveTopology::LineList);

    let surface_vertices: Vec<VertexNC> = surface.surface_vertices(colormap, false);
    let faces = TwoSideLightAux::example(include_str!("../ch09/shader.wgsl"), &surface_vertices);
    
    let axes_vertices: Vec<VertexC> = axes_vertices(2.5);
    let axes = TwoSideLightAux::example(include_str!("mesh.wgsl"), &axes_vertices)
        .with_topology(wgpu::PrimitiveTopology::LineList);

    RenderPassConfiguration::new(vec![mesh, faces, axes])
        .run_title(format!("Chapter 12. Surfaces ({})", surface.name).as_str())
}

fn axes_vertices(scale: f32) -> Vec<VertexC> {
    vec![
        VertexC::new((-scale, 0.0, 0.0), (0.5, 0.0, 0.0)),
        VertexC::new(( scale, 0.0, 0.0), (1.0, 0.5, 0.5)),
        VertexC::new((0.0, -scale, 0.0), (0.0, 0.5, 0.0)),
        VertexC::new((0.0,  scale, 0.0), (0.5, 1.0, 0.5)),
        VertexC::new((0.0, 0.0, -scale), (0.0, 0.0, 0.5)),
        VertexC::new((0.0, 0.0,  scale), (0.5, 0.5, 1.0)),
    ]
}
