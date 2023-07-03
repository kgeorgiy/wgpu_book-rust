use core::iter::zip;

use crate::common::{ColorLight, VertexN};
use crate::common::surface_data::Quads;
use crate::common::vertex_data::{FACE_COLORS_CUBE, i8_as_f32};

mod common;

#[allow(clippy::indexing_slicing)]
fn create_quads() -> Quads<VertexN> {
    zip(i8_as_f32(FACE_COLORS_CUBE.positions), i8_as_f32(FACE_COLORS_CUBE.normals))
        .map(|(positions, normals)|
                 [0, 1, 2, 3].map(|i| VertexN::new(positions[i], normals[i])))
        .into()
}

fn main() {
    ColorLight::example(create_quads().triangles())
        .run_title("Chapter 8. Cube");
}
