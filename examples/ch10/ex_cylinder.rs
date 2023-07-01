use crate::common::run_example;
use crate::common::vertex_data::Cylinder;

mod common;

fn main() {
    run_example("Chapter 10. Cylinder", Cylinder::quads(0.8, 1.5, 2.0, 50, 1.0, 0.5).triangles());
}
