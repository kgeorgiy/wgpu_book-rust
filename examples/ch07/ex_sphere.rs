use cgmath::Deg;

use crate::common::common07::{Vertex, Wireframe};
use crate::common::vertex_data::sphere_position;

#[path = "../common/common.rs"]
mod common;

fn sphere_vertex(r: f32, theta: Deg<f32>, phi: Deg<f32>) -> Vertex {
    Vertex::new(sphere_position(r, theta, phi))
}

fn create_mesh(r: f32, u: usize, v: usize) -> Wireframe {
    let d_theta = Deg(180.0 / u as f32);
    let d_phi = Deg(360.0 / v as f32);

    let mut mesh = Wireframe::new(2 * u * v);
    for i in 0..u {
        for j in 0..v {
            let theta = d_theta * i as f32;
            let phi = d_phi * j as f32;
            let theta1 = d_theta * (i + 1) as f32;
            let phi1 = d_phi * (j + 1) as f32;
            let v0 = sphere_vertex(r, theta, phi);
            let v1 = sphere_vertex(r, theta1, phi);
            let v2 = sphere_vertex(r, theta, phi1);
            mesh.add_lines(&[(v0, v1), (v0, v2)]);
        }
    }
    mesh
}

fn main() {
    create_mesh(1.7, 6, 6).show("Chapter 7. Sphere");
}
