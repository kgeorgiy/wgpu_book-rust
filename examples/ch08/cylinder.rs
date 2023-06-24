use cgmath::{Deg, point3, Point3, vec3};

use crate::common::{ProtoUniforms, Vertex};
use crate::vertex_data::cylinder_position;

#[path = "../common/vertex_data.rs"]
mod vertex_data;
mod common;

pub fn cylinder_vertices(rin: f32, rout: f32, height: f32, n:usize) -> Vec<Vertex> {
    let h = height / 2.0;
    let d_theta = Deg(360.0 / n as f32);

    let up = vec3(0.0, 1.0, 0.0);
    let top_center = point3(0.0, h, 0.0);
    let bot_center = point3(0.0, -h, 0.0);

    let mut vertices: Vec<Vertex> = Vec::with_capacity(24 * n);
    for i in 0..n {
        let theta = d_theta * i as f32;
        let theta1 = d_theta * (i + 1) as f32;

        let top_out = cylinder_position(rout, h, theta);
        let bot_out = cylinder_position(rout, -h, theta);
        let bot_in = cylinder_position(rin, -h, theta);
        let top_in = cylinder_position(rin, h, theta);
        let top_out_1 = cylinder_position(rout, h, theta1);
        let bot_out_1 = cylinder_position(rout, -h, theta1);
        let bot_in_1 = cylinder_position(rin, -h, theta1);
        let top_in_1 = cylinder_position(rin, h, theta1);

        // top face
        vertices.push(Vertex::new(top_out, up));
        vertices.push(Vertex::new(top_out_1, up));
        vertices.push(Vertex::new(top_in_1, up));
        vertices.push(Vertex::new(top_in_1, up));
        vertices.push(Vertex::new(top_in, up));
        vertices.push(Vertex::new(top_out, up));

        // bottom face
        vertices.push(Vertex::new(bot_out, -up));
        vertices.push(Vertex::new(bot_in, -up));
        vertices.push(Vertex::new(bot_in_1, -up));
        vertices.push(Vertex::new(bot_in_1, -up));
        vertices.push(Vertex::new(bot_out_1, -up));
        vertices.push(Vertex::new(bot_out, -up));

        // outer face
        vertices.push(outer(top_out, top_center));
        vertices.push(outer(bot_out, bot_center));
        vertices.push(outer(bot_out_1, bot_center));
        vertices.push(outer(bot_out_1, bot_center));
        vertices.push(outer(top_out_1, top_center));
        vertices.push(outer(top_out, top_center));

        // inner face
        vertices.push(inner(bot_in, bot_center));
        vertices.push(inner(top_in, top_center));
        vertices.push(inner(top_in_1, top_center));
        vertices.push(inner(top_in_1, top_center));
        vertices.push(inner(bot_in_1, bot_center));
        vertices.push(inner(bot_in, bot_center));
    }
    vertices
}

fn inner(p: Point3<f32>, center: Point3<f32>) -> Vertex {
    Vertex::new(p, center - p)
}

fn outer(p: Point3<f32>, center: Point3<f32>) -> Vertex {
    Vertex::new(p, p - center)
}

fn main() {
    ProtoUniforms::example().run("Ch. 8. Cylinder", &cylinder_vertices(0.5, 1.5, 1.5, 30));
}
