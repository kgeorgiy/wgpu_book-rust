use cgmath::point3;

use crate::common::{run_example, VertexNT};
use crate::common::vertex_data::sphere_faces;

mod common;

fn main() {
    run_example("Chapter 10. Sphere", sphere_faces(point3(0.0, 0.0, 0.0), 1.7, 30, 50, &VertexNT::new));
}
