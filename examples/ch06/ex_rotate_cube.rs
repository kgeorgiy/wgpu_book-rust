use webgpu_book::PipelineConfiguration;

use crate::common::create_vertices;
use crate::common::mvp::AnimationState;
use crate::common::vertex_data::FACE_COLORS_CUBE;

mod common;

fn main() {
    PipelineConfiguration::new(include_str!("cube_face_colors.wgsl"))
        .with(AnimationState::example())
        .with_vertices(create_vertices(FACE_COLORS_CUBE.positions, FACE_COLORS_CUBE.colors))
        .run_title("Chapter 6 Auto-rotated cube");
}
