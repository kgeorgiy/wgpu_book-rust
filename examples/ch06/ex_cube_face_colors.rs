use crate::common::{create_vertices, run_example};
use crate::common::vertex_data::FACE_COLORS_CUBE;

mod common;

fn main() {
    run_example(
        "Chapter 6 Solid face colors cube",
        include_str!("cube_face_colors.wgsl"),
        &create_vertices(FACE_COLORS_CUBE.positions, FACE_COLORS_CUBE.colors),
        wgpu::PrimitiveTopology::TriangleList,
        None,
    );
}
