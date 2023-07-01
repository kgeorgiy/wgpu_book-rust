use core::iter::zip;

use crate::common::{run_example, VertexC};
use crate::common::vertex_data::{CUBE_INDEX_DATA, i8_as_f32};

mod common;

#[allow(clippy::indexing_slicing)]
fn main() {
    let cube = CUBE_INDEX_DATA;
    let vertices: Vec<VertexC> = zip(i8_as_f32([cube.positions])[0], i8_as_f32([cube.colors])[0])
        .map(|(pos, col)| VertexC::new(pos, col))
        .collect();
    run_example(
        "Chapter 6 Vertex colors cube",
        include_str!("cube_face_colors.wgsl"),
        vertices,
        wgpu::PrimitiveTopology::TriangleList,
        Some(&cube.indices),
    );
}
