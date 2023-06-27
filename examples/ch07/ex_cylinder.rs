use cgmath::Deg;

use crate::common::{cylinder_vertex, Wireframe};

mod common;

fn create_mesh(rin: f32, rout: f32, height: f32, n: usize) -> Wireframe {
    let h = height / 2.0;
    let d_theta = Deg(360.0 / n as f32);

    let mut mesh = Wireframe::new(3 * n);
    for i in 0..n {
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

        // top face
        mesh.add_lines(&[(top_out, top_in), (top_in, top_in_1), (top_out_1, top_out)]);
        // bottom face
        mesh.add_lines(&[(bot_out, bot_in), (bot_in, bot_in_1), (bot_out_1, bot_out)]);
        // side
        mesh.add_lines(&[(top_out, bot_out), (top_in, bot_in)]);
    }
    mesh
}

fn main() {
    create_mesh(0.5, 1.0, 2.5, 20).into_config().run_title("Chapter 7. Cylinder");
}
