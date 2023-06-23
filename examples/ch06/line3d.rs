use std::f32::consts::PI;

use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, Rad, SquareMatrix, Vector3};
use wgpu::{PrimitiveTopology, VertexAttribute};

use state::State;
use webgpu_book::VertexBufferInfo;

mod state;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
    pub position: [f32; 3],
}

impl VertexBufferInfo for Vertex {
    const ATTRIBUTES: &'static [VertexAttribute] = &wgpu::vertex_attr_array![0=>Float32x3];
}

fn create_vertices() -> [Vertex; 300] {
    let mut vertices = [Vertex {
        position: [0.0, 0.0, 0.0],
    }; 300];
    for i in 0..300 {
        let t = 0.1 * (i as f32) / 30.0;
        let x = (-t).exp() * (30.0 * t).sin();
        let z = (-t).exp() * (30.0 * t).cos();
        let y = 2.0 * t - 1.0;
        vertices[i] = Vertex {
            position: [x, y, z],
        };
    }
    vertices
}

fn main() {
    State::run(
        "Line",
        include_str!("line3d.wgsl"),
        (1.5, 1.0, 3.0).into(),
        (0.0, 0.0, 0.0).into(),
        Vector3::unit_y(),
        Rad(2.0 * PI / 5.0),
        0.0,
        Matrix4::identity(),
        &create_vertices(),
        PrimitiveTopology::LineStrip,
        None,
    );
}
