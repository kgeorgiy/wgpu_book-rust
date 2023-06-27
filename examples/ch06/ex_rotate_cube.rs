use crate::common::create_vertices;
use crate::common::mvp::{AnimationState, MvpMatrix};
use crate::common::vertex_data::FACE_COLORS_CUBE;

mod common;

fn main() {
    AnimationState::example_config::<MvpMatrix>(include_str!("cube_face_colors.wgsl"))
        .with_vertices(&create_vertices(FACE_COLORS_CUBE.positions, FACE_COLORS_CUBE.colors))
        .run_title("Chapter 6 Auto-rotated cube");
}
