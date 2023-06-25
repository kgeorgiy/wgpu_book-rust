use crate::common::{ProtoUniforms, Vertex};
use crate::vertex_data::FACE_COLORS_CUBE;

#[path = "../common/vertex_data.rs"]
mod vertex_data;
mod common;

fn create_vertices() -> Vec<Vertex> {
    let positions = FACE_COLORS_CUBE.positions;
    let normals = FACE_COLORS_CUBE.normals;

    let mut data: Vec<Vertex> = Vec::with_capacity(positions.len());
    for i in 0..positions.len() {
        let position = positions[i];
        let normal = normals[i];
        data.push(Vertex {
            position: [position[0] as f32, position[1] as f32, position[2] as f32, 1.0],
            normal: [normal[0] as f32, normal[1] as f32, normal[2] as f32, 1.0],
        });
    }
    data.to_vec()
}

fn main() {
    ProtoUniforms::example().run("Ch. 8. Cube", &create_vertices());
}
