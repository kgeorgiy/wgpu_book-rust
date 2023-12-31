use core::iter::zip;

use crate::common::{run_example, VertexNT};
use crate::common::surface_data::Quads;
use crate::common::vertex_data::{i8_as_f32, MULTI_TEXTURE_CUBE};

mod common;

#[allow(clippy::indexing_slicing)]
fn create_cube() -> Quads<VertexNT> {
    let cube = MULTI_TEXTURE_CUBE;
    zip(i8_as_f32(cube.positions), zip(i8_as_f32(cube.normals), cube.uvs))
        .map(|(position, (normal, uvs))|
            [0, 1, 2, 3].map(|i| VertexNT::new(position[i], normal[i], uvs[i])))
        .into()
}

fn main() {
    run_example("Chapter 10. Multi-textured cube", create_cube().triangles());
}