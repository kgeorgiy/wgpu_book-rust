use crate::common::{ColorLight};
use crate::common::vertex_data::Torus;

mod common;

fn main() {
    ColorLight::example(Torus::quads(1.5, 0.4, 20, 20).triangles())
        .run_title("Chapter 8. Torus")
}
