use cgmath::{Deg, point3};

use crate::common::{ProtoUniforms, Vertex};
use crate::vertex_data::sphere_position;

#[path = "../common/vertex_data.rs"]
mod vertex_data;
mod common;

fn sphere_vertex(r: f32, theta: Deg<f32>, phi: Deg<f32>) -> Vertex {
    let center = point3(0.0, 0.0, 0.0);
    let position = sphere_position(r, theta, phi);
    Vertex::new(position, (position - center) / r)
}

pub fn sphere_vertices(r: f32, u: usize, v: usize) -> Vec<Vertex> {
    let d_theta = Deg(180.0 / u as f32);
    let d_phi = Deg(360.0 / v as f32);

    let mut vertices: Vec<Vertex> = Vec::with_capacity(4 * u * v);
    for i in 0..u {
        for j in 0..v {
            let theta = d_theta * i as f32;
            let phi = d_phi * j as f32;
            let theta1 = d_theta * (i + 1) as f32;
            let phi1 = d_phi * (j + 1) as f32;
            let p0 = sphere_vertex(r, theta, phi);
            let p1 = sphere_vertex(r, theta1, phi);
            let p2 = sphere_vertex(r, theta1, phi1);
            let p3 = sphere_vertex(r, theta, phi1);

            vertices.extend_from_slice(&[p0, p1, p3]);
            vertices.extend_from_slice(&[p1, p2, p3]);
        }
    }
    vertices
}

fn main() {
    ProtoUniforms::example().run("Ch. 8. Cube", &sphere_vertices(1.5, 10, 20));
}
