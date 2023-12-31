use crate::common::ColorLight;
use crate::common::vertex_data::Cone;

mod common;

fn main() {
    ColorLight::example(Cone::triangles(0.5, 1.5, 2.0, 12))
        .run_title("Chapter 8. Cone");
}
