use bytemuck::{Pod, Zeroable};
use wgpu::{PrimitiveTopology, VertexAttribute};

use webgpu_book::VertexBufferInfo;

use crate::common06::Mvp;

mod common06;

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
    Mvp::run(
        "Ch6. Line",
        include_str!("line3d.wgsl"),
        &create_vertices(),
        PrimitiveTopology::LineStrip,
        None,
    );
}
