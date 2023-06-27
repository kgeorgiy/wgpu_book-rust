use webgpu_book::VertexBufferInfo;

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
    const ATTRIBUTES: &'static [wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![0=>Float32x2, 1=>Float32x3];
}


pub fn run_example(title: &str, vertices: &[Vertex], indices: Option<&[u16]>) -> ! {
    Config::with_vertices::<Vertex, u16, 0>(include_str!("triangle.wgsl"), vertices, indices)
        .run_title(title)
}
