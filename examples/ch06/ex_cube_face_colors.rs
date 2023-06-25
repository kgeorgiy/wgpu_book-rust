use wgpu::PrimitiveTopology;

use crate::common06::{ColorVertex, Mvp};
use crate::vertex_data::FACE_COLORS_CUBE;

#[path = "../common/vertex_data.rs"]
mod vertex_data;
mod common06;

fn main() {
    Mvp::run(
        "Ch6. Solid face colors cube",
        include_str!("cube_face_colors.wgsl"),
        &ColorVertex::create(FACE_COLORS_CUBE.positions, FACE_COLORS_CUBE.colors),
        PrimitiveTopology::TriangleList,
        None,
    );
}
