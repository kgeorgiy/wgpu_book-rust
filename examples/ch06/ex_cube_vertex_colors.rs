use wgpu::PrimitiveTopology;

use crate::common06::{ColorVertex, Mvp};
use crate::vertex_data::CUBE_INDEX_DATA;

#[path = "../common/vertex_data.rs"]
mod vertex_data;
mod common06;

fn main() {
    Mvp::run(
        "Ch6. Vertex colors cube",
        include_str!("cube_face_colors.wgsl"),
        &ColorVertex::create(CUBE_INDEX_DATA.positions, CUBE_INDEX_DATA.colors),
        PrimitiveTopology::TriangleList,
        Some(&CUBE_INDEX_DATA.indices),
    );
}
