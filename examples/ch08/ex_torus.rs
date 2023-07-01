use crate::common::{LightAux};
use crate::common::vertex_data::Torus;

mod common;

fn main() {
    LightAux::example(Torus::quads(1.5, 0.4, 20, 20).triangles())
        .run_title("Chapter 8. Torus")
}
