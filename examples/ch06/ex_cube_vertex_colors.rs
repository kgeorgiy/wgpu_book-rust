use crate::common::{create_vertices, run_example};
use crate::common::vertex_data::CUBE_INDEX_DATA;

mod common;

fn main() {
    run_example(
        "Chapter 6 Vertex colors cube",
        include_str!("cube_face_colors.wgsl"),
        create_vertices(CUBE_INDEX_DATA.positions, CUBE_INDEX_DATA.colors),
        wgpu::PrimitiveTopology::TriangleList,
        Some(&CUBE_INDEX_DATA.indices),
    );
}
