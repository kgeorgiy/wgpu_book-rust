use cgmath::{Deg, point3, Point3};

use crate::common::{run_example, VertexNT};
use crate::common::vertex_data::{sphere_position, sphere_vertices};

mod common;

const CENTER: Point3<f32> = point3(0.0, 0.0, 0.0);

fn sphere_vertex(r: f32, theta: Deg<f32>, phi: Deg<f32>) -> VertexNT {
    let position = sphere_position(r, theta, phi);
    VertexNT::new(position, (position - CENTER) / r, (phi.0 / 360.0, theta.0 / 180.0))
}

fn main() {
    run_example("Chapter 10. Sphere", sphere_vertices(1.7, 30, 50, sphere_vertex));
}
