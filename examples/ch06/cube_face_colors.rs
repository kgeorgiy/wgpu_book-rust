use std::f32::consts::PI;

use cgmath::{Matrix4, Rad, SquareMatrix, Vector3};
use wgpu::PrimitiveTopology;

use state::State;

use crate::state::ColorVertex;
use crate::vertex_data::FACE_COLORS_CUBE;

#[path = "../common/vertex_data.rs"]
mod vertex_data;
mod state;

fn main() {
    State::run(
        "Solid face colors cube",
        include_str!("cube_face_colors.wgsl"),
        (3.0, 1.5, 3.0).into(),
        (0.0, 0.0, 0.0).into(),
        Vector3::unit_y(),
        Rad(2.0 * PI / 5.0),
        0.0,
        Matrix4::identity(),
        &ColorVertex::create(FACE_COLORS_CUBE.positions, FACE_COLORS_CUBE.colors),
        PrimitiveTopology::TriangleList,
        None,
    );
}
