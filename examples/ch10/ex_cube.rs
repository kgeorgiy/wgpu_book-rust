use core::iter::zip;

use crate::common::{run_example, VertexNT};
use crate::common::vertex_data::{FACE_COLORS_CUBE, i8_as_f32};

mod common;

fn create_vertices() -> Vec<VertexNT> {
    #![allow(clippy::indexing_slicing)]
    let cube = FACE_COLORS_CUBE;
    zip(i8_as_f32(cube.positions), zip(i8_as_f32(cube.normals), i8_as_f32(cube.uvs)))
        .map(|(position, (normal, uv))| VertexNT::new(position, normal, uv))
        .collect()
}

fn main() {
    run_example("Chapter 10. Cube", create_vertices());
}