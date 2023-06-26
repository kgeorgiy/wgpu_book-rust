use std::iter::zip;

use crate::common::{LightAux, VertexN};
use crate::common::vertex_data::{FACE_COLORS_CUBE, i8_as_f32};

mod common;

fn create_vertices() -> Vec<VertexN> {
    zip(i8_as_f32(FACE_COLORS_CUBE.positions), i8_as_f32(FACE_COLORS_CUBE.normals))
        .map(|(position, normal)| VertexN::new(position, normal))
        .collect()
}

fn main() {
    LightAux::example().run("Chapter 8. Cube", &create_vertices());
}
