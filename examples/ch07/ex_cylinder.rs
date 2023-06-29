use cgmath::Deg;
use common::surface_data::Mesh;

use crate::common::{cylinder_vertex, Vertex};
use crate::common::mvp::AnimationState;

mod common;

fn create_mesh(rin: f32, rout: f32, height: f32, n: usize) -> Mesh<Vertex> {
    let h = height / 2.0;
    let d_theta = Deg(360.0 / n as f32);

    Mesh::from((0..n).flat_map(|i| {
        let theta = d_theta * i as f32;
        let theta1 = d_theta * (i + 1) as f32;

        let top_out = cylinder_vertex(rout, h, theta);
        let bot_out = cylinder_vertex(rout, -h, theta);
        let bot_in = cylinder_vertex(rin, -h, theta);
        let top_in = cylinder_vertex(rin, h, theta);
        let top_out_1 = cylinder_vertex(rout, h, theta1);
        let bot_out_1 = cylinder_vertex(rout, -h, theta1);
        let bot_in_1 = cylinder_vertex(rin, -h, theta1);
        let top_in_1 = cylinder_vertex(rin, h, theta1);

        [
            (top_out, top_in), (top_in, top_in_1), (top_out_1, top_out), // top face
            (bot_out, bot_in), (bot_in, bot_in_1), (bot_out_1, bot_out), // bottom face
            (top_out, bot_out), (top_in, bot_in), // side
        ]
    }))
}

fn main() {
    create_mesh(0.5, 1.0, 2.5, 20).into_config()
        .with(AnimationState::example())
        .run_title("Chapter 7. Cylinder");
}
