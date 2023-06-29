use cgmath::Deg;
use common::surface_data::Mesh;

use crate::common::{cylinder_vertex, Vertex};
use crate::common::mvp::AnimationState;

mod common;

fn create_mesh(top_r: f32, bot_r: f32, height: f32, n: usize) -> Mesh<Vertex> {
    let h = height / 2.0;
    let d_theta = Deg(360.0 / n as f32);

    Mesh::from((0..n).flat_map(|i| {
        let theta = d_theta * i as f32;
        let theta1 = d_theta * (i + 1) as f32;
        let top = cylinder_vertex(top_r, h, theta);
        let bot = cylinder_vertex(bot_r, -h, theta);
        let bot_center = cylinder_vertex(0.0, -h, theta);
        let top_center = cylinder_vertex(0.0, h, theta);
        let top_1 = cylinder_vertex(top_r, h, theta1);
        let bot_1 = cylinder_vertex(bot_r, -h, theta1);

        [
            (top, top_center), (top_1, top),    // top face 2 lines
            (bot, bot_center), (bot_1, bot),    // bottom face 2 lines
            (top, bot)                          // side 1 line
        ]
    }))
}

fn main() {
    create_mesh(0.5, 1.5, 2.0, 20).into_config()
        .with(AnimationState::example())
        .run_title("Chapter 7. Cone");
}
