use std::f32::consts::PI;

use cgmath::{Matrix4, Rad, SquareMatrix, Vector3};
use wgpu::PrimitiveTopology;

use state::{ColorVertex, State};

use crate::vertex_data::CUBE_INDEX_DATA;

#[path = "../common/vertex_data.rs"]
mod vertex_data;
mod state;

fn main() {
    State::run(
        "Vertex colors cube",
        include_str!("cube_face_colors.wgsl"),
        (3.0, 1.5, 3.0).into(),
        (0.0, 0.0, 0.0).into(),
        Vector3::unit_y(),
        Rad(2.0 * PI / 5.0),
        0.0,
        Matrix4::identity(),
        &ColorVertex::create(CUBE_INDEX_DATA.positions, CUBE_INDEX_DATA.colors),
        PrimitiveTopology::TriangleList,
        Some(&CUBE_INDEX_DATA.indices),
    );
}
