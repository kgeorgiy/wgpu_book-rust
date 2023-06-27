use core::iter::zip;

use crate::common::{run_example, VertexNT};
use crate::common::vertex_data::{i8_as_f32, MULTI_TEXTURE_CUBE};

mod common;

fn create_vertices() -> Vec<VertexNT> {
    let cube = MULTI_TEXTURE_CUBE;
    zip(i8_as_f32(cube.positions), zip(i8_as_f32(cube.normals), cube.uvs))
        .map(|(position, (normal, uvs))| VertexNT::new(position, normal, uvs))
        .collect()
}

fn main() {
    run_example("Chapter 10. Multi-textured cube", &create_vertices());
}