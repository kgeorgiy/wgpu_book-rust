use cgmath::point3;

use crate::common::{ColorLightAux, VertexN};
use crate::common::vertex_data::sphere_triangles;

mod common;

fn main() {
    ColorLightAux::example(sphere_triangles(point3(0.0, 0.0, 0.0), 1.5, 10, 20, &|position, normal, _lat_lon| VertexN::new(position, normal)))
        .run_title("Chapter 8. Sphere");
}
