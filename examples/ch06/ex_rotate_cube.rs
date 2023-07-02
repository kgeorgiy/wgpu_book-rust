use webgpu_book::PipelineConfiguration;

use crate::common::create_cube;
use crate::common::mvp::AnimationState;
use crate::common::vertex_data::FACE_COLORS_CUBE;

mod common;

fn main() {
    let cube = FACE_COLORS_CUBE;
    PipelineConfiguration::new(include_str!("cube_face_colors.wgsl"))
        .with(AnimationState::example())
        .with(create_cube(cube.positions, cube.colors).triangles().vertices())
        .run_title("Chapter 6 Auto-rotated cube");
}
