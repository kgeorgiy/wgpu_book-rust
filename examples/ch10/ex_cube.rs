use core::iter::zip;

use crate::common::{run_example, VertexNT};
use crate::common::surface_data::Quads;
use crate::common::vertex_data::{FACE_COLORS_CUBE, i8_as_f32};

mod common;


fn create_cube() -> Quads<VertexNT> {
    #![allow(clippy::indexing_slicing)]
    let cube = FACE_COLORS_CUBE;
    zip(i8_as_f32(cube.positions), zip(i8_as_f32(cube.normals), i8_as_f32(cube.uvs)))
        .map(|(position, (normal, uvs))|
            [0, 1, 2, 3].map(|i| VertexNT::new(position[i], normal[i], uvs[i])))
        .into()
}


fn main() {
    run_example("Chapter 10. Cube", create_cube().triangles());
}