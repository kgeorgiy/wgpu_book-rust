use crate::common::{AnimationState, create_vertices, MvpMatrix, VertexC};
use crate::common::vertex_data::FACE_COLORS_CUBE;

mod common;

fn main() {
    AnimationState::run::<MvpMatrix, VertexC, u16>(
        "Chapter 6 Auto-rotated cube",
        include_str!("cube_face_colors.wgsl"),
        1.0,
        &create_vertices(FACE_COLORS_CUBE.positions, FACE_COLORS_CUBE.colors),
        wgpu::PrimitiveTopology::TriangleList,
        None
    );
}
