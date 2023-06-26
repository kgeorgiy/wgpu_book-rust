use cgmath::{Deg, point3, Point3};

use crate::common::{LightAux, VertexN};
use crate::common::vertex_data::{sphere_position, sphere_vertices};

mod common;

const CENTER: Point3<f32> = point3(0.0, 0.0, 0.0);

fn sphere_vertex(r: f32, theta: Deg<f32>, phi: Deg<f32>) -> VertexN {
    let position = sphere_position(r, theta, phi);
    VertexN::new(position, (position - CENTER) / r)
}

fn main() {
    LightAux::example().run("Chapter 8. Sphere", &sphere_vertices(1.5, 10, 20, sphere_vertex));
}
