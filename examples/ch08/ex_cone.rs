use cgmath::{Deg, point3, Point3, vec3};

use crate::common::{LightAux, VertexN};
use crate::common::vertex_data::cylinder_position;

mod common;

fn cone_vertices(r_top: f32, r_bottom: f32, height: f32, n: usize) -> Vec<VertexN> {
    let h = height / 2.0;
    let d_theta = Deg(360.0 / n as f32);

    let up = vec3(0.0, 1.0, 0.0);

    let mut vertices: Vec<VertexN> = Vec::with_capacity(12 * n);
    for i in 0..n {
        let theta = d_theta * i as f32;
        let theta_1 = d_theta * (i + 1) as f32;

        let top_out = cylinder_position(r_top, h, theta);
        let bot_out = cylinder_position(r_bottom, -h, theta);
        let bot_cen = cylinder_position(0.0, -h, theta);
        let top_cen = cylinder_position(0.0, h, theta);
        let top_out_1 = cylinder_position(r_top, h, theta_1);
        let bot_out_1 = cylinder_position(r_bottom, -h, theta_1);

        // top face
        vertices.push(VertexN::new(top_out, up));
        vertices.push(VertexN::new(top_out_1, up));
        vertices.push(VertexN::new(top_cen, up));

        // bottom face
        vertices.push(VertexN::new(bot_out, -up));
        vertices.push(VertexN::new(bot_cen, -up));
        vertices.push(VertexN::new(bot_out_1, -up));

        // outer face
        vertices.push(outer(top_out, bot_out));
        vertices.push(outer(bot_out, top_out));
        vertices.push(outer(bot_out_1, top_out_1));

        vertices.push(outer(bot_out_1, top_out_1));
        vertices.push(outer(top_out_1, bot_out_1));
        vertices.push(outer(top_out, bot_out));
    }
    vertices
}

const ORIGIN: Point3<f32> = point3(0.0, 0.0, 0.0);

fn outer(p: Point3<f32>, other: Point3<f32>) -> VertexN {
    let dp = other - p;
    VertexN::new(p, (ORIGIN - p).cross(dp).cross(dp))
}


fn main() {
    LightAux::example(cone_vertices(0.5, 1.5, 2.0, 12))
        .run_title("Chapter 8. Cone");
}
