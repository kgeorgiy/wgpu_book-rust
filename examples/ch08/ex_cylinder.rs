use crate::common::{ColorLightAux, VertexN};
use crate::common::vertex_data::Cylinder;

mod common;

fn main() {
    let triangles = Cylinder::quads(0.5, 1.5, 1.5, 30, 0.0, 0.0).cast::<VertexN>().triangles();
    ColorLightAux::example(triangles)
        .with_cull_mode(Some(wgpu::Face::Back))
        .run_title("Chapter 8. Cylinder");
}
