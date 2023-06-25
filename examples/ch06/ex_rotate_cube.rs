use wgpu::PrimitiveTopology;

use crate::common06::{AnimationState, ColorVertex};
use crate::vertex_data::FACE_COLORS_CUBE;

#[path = "../common/vertex_data.rs"]
mod vertex_data;
mod common06;

fn main() {
    AnimationState::run(
        "Ch6. Auto-rotated cube",
        include_str!("cube_face_colors.wgsl"),
        1.0,
        &ColorVertex::create(FACE_COLORS_CUBE.positions, FACE_COLORS_CUBE.colors),
        PrimitiveTopology::TriangleList,
        None,
    );
}
