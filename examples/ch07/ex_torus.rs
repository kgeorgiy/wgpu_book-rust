use cgmath::Deg;
use webgpu_book::PipelineConfiguration;

use self::common::{Vertex, Wireframe};
use crate::common::vertex_data::torus_position;

#[allow(clippy::duplicate_mod)]
mod common;

pub(crate) fn torus_vertex(r_torus: f32, r_tube: f32, u: Deg<f32>, v: Deg<f32>) -> Vertex {
    Vertex::new(torus_position(r_torus, r_tube, u, v))
}

fn create_mesh(r_torus: f32, r_tube: f32, n_torus: usize, n_tube: usize) -> Wireframe {
    let d_u = Deg(360.0 / n_torus as f32);
    let d_v = Deg(360.0 / n_tube as f32);

    let mut mesh = Wireframe::new(2 * n_torus* n_tube);
    for i in 0..n_torus {
        for j in 0..n_tube {
            let u = d_u * i as f32;
            let v = d_v * j as f32;
            let u1 = d_u * (i as f32 + 1.0);
            let v1 = d_v * (j as f32 + 1.0);
            let p0 = torus_vertex(r_torus, r_tube, u, v);
            let p1 = torus_vertex(r_torus, r_tube, u1, v);
            let p3 = torus_vertex(r_torus, r_tube, u, v1);
            mesh.add_lines(&[(p3, p0), (p0, p1)]);
        }
    }
    mesh
}

#[must_use] pub fn pipeline() -> PipelineConfiguration {
    create_mesh(1.5, 0.3, 20, 10).into_config()
}

#[allow(dead_code)]
fn main() {
    pipeline().run_title("Chapter 7. Torus");
}
