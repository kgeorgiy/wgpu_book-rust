use cgmath::Deg;
use common::surface_data::Mesh;
use crate::common::mvp::AnimationState;

use crate::common::Vertex;
use crate::common::vertex_data::sphere_position;

mod common;

fn sphere_vertex(r: f32, theta: Deg<f32>, phi: Deg<f32>) -> Vertex {
    Vertex::new(sphere_position(r, theta, phi))
}

fn create_mesh(r: f32, u: usize, v: usize) -> Mesh<Vertex> {
    let d_theta = Deg(180.0 / u as f32);
    let d_phi = Deg(360.0 / v as f32);

    Mesh::from((0..u).flat_map(|i| (0..v).flat_map(move |j| {
        let theta = d_theta * i as f32;
        let phi = d_phi * j as f32;
        let theta1 = d_theta * (i + 1) as f32;
        let phi1 = d_phi * (j + 1) as f32;
        let v0 = sphere_vertex(r, theta, phi);
        let v1 = sphere_vertex(r, theta1, phi);
        let v2 = sphere_vertex(r, theta, phi1);
        [(v0, v1), (v0, v2)]
    })))
}

fn main() {
    create_mesh(1.7, 20, 20).into_config()
        .with(AnimationState::example())
        .run_title("Chapter 7. Sphere");
}
