use webgpu_book::{PipelineConfiguration, VertexBufferInfo};

pub use self::global_common::*;

#[path = "../common/global_common.rs"]
mod global_common;

// Vertex with position and color

#[repr(C)]
#[derive(Copy, Clone, Debug, ::bytemuck::Pod, ::bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

impl VertexBufferInfo for Vertex {
    const NAME: &'static str = "Vertex";
    const ATTRIBUTES: &'static [wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![0=>Float32x2, 1=>Float32x3];
    const ATTRIBUTE_NAMES: &'static [&'static str] = &["position", "color"];
}

pub fn run_example(title: &str, vertices: Vec<Vertex>, indices: Option<&[u16]>) -> ! {
    PipelineConfiguration::new(include_str!("triangle.wgsl"))
        .with_vertices_indices(vertices, indices, wgpu::PrimitiveTopology::TriangleList)
        .run_title(title);
}
