use cgmath::point3;

use crate::common::{run_example, VertexNT};
use crate::common::vertex_data::sphere_triangles;

mod common;

fn main() {
    run_example("Chapter 10. Sphere", sphere_triangles(point3(0.0, 0.0, 0.0), 1.7, 30, 50, &VertexNT::new));
}
