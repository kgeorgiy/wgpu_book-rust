use cgmath::{Point2, Point3, Vector3};

use crate::common::common10::{run_example, Vertex};
use crate::common::vertex_data::FACE_COLORS_CUBE;

#[path = "../common/common.rs"]
mod common;

fn vertex(position: [i8; 3], normal: [i8; 3], uv: [i8; 2]) -> Vertex {
    Vertex::new(
        Point3::from(position).cast::<f32>().unwrap(),
        Vector3::from(normal).cast::<f32>().unwrap(),
        Point2::from(uv).cast::<f32>().unwrap(),
    )
}

fn create_vertices() -> Vec<Vertex> {
    let cube = FACE_COLORS_CUBE;
    let mut data: Vec<Vertex> = Vec::with_capacity(cube.positions.len());
    for i in 0..cube.positions.len() {
        data.push(vertex(cube.positions[i], cube.normals[i], cube.uvs[i]));
    }
    data.to_vec()
}

fn main() {
    run_example("Ch 10. Cube", &create_vertices());
}