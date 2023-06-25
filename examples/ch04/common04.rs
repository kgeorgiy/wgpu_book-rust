use bytemuck::{Pod, Zeroable};
use wgpu::VertexAttribute;

use webgpu_book::{BufferInfo, RenderConfiguration, run_wgpu_title, VertexBufferInfo};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

impl VertexBufferInfo for Vertex {
    const ATTRIBUTES: &'static [VertexAttribute] =
        &wgpu::vertex_attr_array![0=>Float32x2, 1=>Float32x3];
}

pub fn run_example(title: &str, vertices: &[Vertex], indices: Option<&[u16]>) -> ! {
    run_wgpu_title(title, RenderConfiguration {
        shader_source: include_str!("triangle.wgsl").to_string(),
        vertices: indices.map_or(vertices.len(), |idx| idx.len()),
        vertex_buffers: vec![Vertex::buffer("Vertices", &vertices)],
        index_buffer: indices.map(|idx| u16::buffer("Indices", idx)),
        ..RenderConfiguration::default()
    })
}
