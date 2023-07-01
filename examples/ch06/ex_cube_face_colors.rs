use webgpu_book::PipelineConfiguration;
use crate::common::{create_cube};
use crate::common::mvp::MvpController;
use crate::common::vertex_data::FACE_COLORS_CUBE;

mod common;

fn main() {
    let quads = create_cube(FACE_COLORS_CUBE.positions, FACE_COLORS_CUBE.colors);
    PipelineConfiguration::new(include_str!("cube_face_colors.wgsl"))
        .with(MvpController::example(()))
        .with_vertices(quads.triangles().vertices())
        .with_topology(wgpu::PrimitiveTopology::TriangleList)
        .run_title("Chapter 6 Solid face colors cube");
}
