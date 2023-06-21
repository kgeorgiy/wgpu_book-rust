use bytemuck::{Pod, Zeroable};
use wgpu::VertexAttribute;

use webgpu_book::VertexBufferInfo;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

impl VertexBufferInfo for Vertex {
    const ATTRIBUTES: &'static [VertexAttribute] = &wgpu::vertex_attr_array![0=>Float32x2, 1=>Float32x3];
}
