use cgmath::Deg;

use crate::common::common07::{cylinder_vertex, Wireframe};

#[path = "../common/common.rs"]
mod common;

fn create_mesh(top_r: f32, bot_r: f32, height: f32, n: usize) -> Wireframe {
    let h = height / 2.0;
    let d_theta = Deg(360.0 / n as f32);

    let mut mesh = Wireframe::new(5 * n);
    for i in 0..n {
        let theta = d_theta * i as f32;
        let theta1 = d_theta * (i + 1) as f32;
        let top = cylinder_vertex(top_r, h, theta);
        let bot = cylinder_vertex(bot_r, -h, theta);
        let bot_center = cylinder_vertex(0.0, -h, theta);
        let top_center = cylinder_vertex(0.0, h, theta);
        let top_1 = cylinder_vertex(top_r, h, theta1);
        let bot_1 = cylinder_vertex(bot_r, -h, theta1);

        // top face 2 lines
        mesh.add_lines(&[(top, top_center), (top_1, top)]);
        // bottom face 2 lines
        mesh.add_lines(&[(bot, bot_center), (bot_1, bot)]);
        // side 1 line
        mesh.add_line(top, bot);
    }
    mesh
}

fn main() {
    create_mesh(0.5, 1.5, 2.0, 20).show("Chapter 7. Cone");
}
