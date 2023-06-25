use cgmath::{Deg, point2, point3, vec3, Vector3};

use crate::common::common10::{run_example, Vertex};
use crate::common::vertex_data::cylinder_position;

#[path = "../common/common.rs"]
mod common;

pub fn cylinder_vertices(rin: f32, rout: f32, height: f32, n: usize, ul: f32, vl: f32) -> Vec<Vertex> {
    let h = height / 2.0;
    let d_theta = Deg(360.0 / n as f32);

    let top = h;
    let bot = -h;
    let up = vec3(0.0, 1.0, 0.0);
    let du = ul / 360.0;
    let dl = vl / h / 2.0;
    let face_params = (ul, rin, vl / height);

    let mut vertices: Vec<Vertex> = Vec::with_capacity(24 * n);
    for i in 0..n {
        let theta_1 = d_theta * i as f32;
        let theta_2 = d_theta * (i + 1) as f32;

        // top face
        vertices.push(face(rout, top, theta_1, up, face_params));
        vertices.push(face(rout, top, theta_2, up, face_params));
        vertices.push(face(rin , top, theta_2, up, face_params));
        vertices.push(face(rin , top, theta_2, up, face_params));
        vertices.push(face(rin , top, theta_1, up, face_params));
        vertices.push(face(rout, top, theta_1, up, face_params));

        // bottom face
        vertices.push(face(rout, bot, theta_1, -up, face_params));
        vertices.push(face(rin , bot, theta_1, -up, face_params));
        vertices.push(face(rin , bot, theta_2, -up, face_params));
        vertices.push(face(rin , bot, theta_2, -up, face_params));
        vertices.push(face(rout, bot, theta_2, -up, face_params));
        vertices.push(face(rout, bot, theta_1, -up, face_params));

        // outer face
        vertices.push(side(top, theta_1, rout, 1.0, h, du, dl));
        vertices.push(side(bot, theta_1, rout, 1.0, h, du, dl));
        vertices.push(side(bot, theta_2, rout, 1.0, h, du, dl));
        vertices.push(side(bot, theta_2, rout, 1.0, h, du, dl));
        vertices.push(side(top, theta_2, rout, 1.0, h, du, dl));
        vertices.push(side(top, theta_1, rout, 1.0, h, du, dl));

        // inner face
        vertices.push(side(bot, theta_1, rin, -1.0, h, du, dl));
        vertices.push(side(top, theta_1, rin, -1.0, h, du, dl));
        vertices.push(side(top, theta_2, rin, -1.0, h, du, dl));
        vertices.push(side(top, theta_2, rin, -1.0, h, du, dl));
        vertices.push(side(bot, theta_2, rin, -1.0, h, du, dl));
        vertices.push(side(bot, theta_1, rin, -1.0, h, du, dl));
    }
    vertices
}

fn face(r: f32, h: f32, theta: Deg<f32>, normal: Vector3<f32>, (ul, rin, vc): (f32, f32, f32)) -> Vertex {
    let u = ul * theta.0 / 360.0;
    let v = vc * (r - rin);
    Vertex::new(cylinder_position(r, h, theta), normal, point2(u, v))
}

fn side(y: f32, theta: Deg<f32>, r: f32, dn: f32, h: f32, du: f32, dv: f32) -> Vertex {
    let p = cylinder_position(r, y, theta);
    Vertex::new(p, (p - point3(0.0, y, 0.0)) * dn, point2(du * theta.0, (y + h) * dv))
}

fn main() {
    run_example("Ch 10. Cylinder", &cylinder_vertices(0.8, 1.5, 2.0, 50, 20.0, 0.5));
}
