use crate::common::LightAux;
use crate::common::vertex_data::Cone;

mod common;

fn main() {
    LightAux::example(Cone::triangles(0.5, 1.5, 2.0, 12))
        .run_title("Chapter 8. Cone");
}
