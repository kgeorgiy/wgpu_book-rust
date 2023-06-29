use cgmath::Deg;

use crate::common::{LightAux, VertexN};
use crate::common::vertex_data::torus_position;

mod common;

fn torus_vertices(r_torus:f32, r_tube:f32, n_torus:usize, n_tube:usize) -> Vec<VertexN> {
    let d_u = Deg(360.0 / n_torus as f32);
    let d_v = Deg(360.0 / n_tube as f32);

    let mut vertices: Vec<VertexN> = Vec::with_capacity(4 * n_torus * n_tube);
    for i in 0..n_torus {
        for j in 0..n_tube {
            let u = d_u * i as f32;
            let v = d_v * j as f32;
            let u1 = d_u * (i as f32 + 1.0);
            let v1 = d_v * (j as f32 + 1.0);

            let p0 = torus_vertex(r_torus, r_tube, u, v);
            let p1 = torus_vertex(r_torus, r_tube, u1, v);
            let p2 = torus_vertex(r_torus, r_tube, u1, v1);
            let p3 = torus_vertex(r_torus, r_tube, u, v1);

            // positions
            vertices.push(p0);
            vertices.push(p1);
            vertices.push(p2);
            vertices.push(p2);
            vertices.push(p3);
            vertices.push(p0);
        }
    }
    vertices
}

fn torus_vertex(r_torus: f32, r_tube: f32, u: Deg<f32>, v: Deg<f32>) -> VertexN {
    let position = torus_position(r_torus, r_tube, u, v);
    let center = torus_position(r_torus, 0.0, u, v);
    VertexN::new(position, position - center)
}

fn main() {
    LightAux::example(torus_vertices(1.5, 0.4, 20, 20))
        .run_title("Chapter 8. Torus")
}
