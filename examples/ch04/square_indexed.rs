use std::u16;

use webgpu_book::{RenderConfiguration, run_wgpu, WindowConfiguration, VertexBufferInfo, BufferInfo};

use crate::vertex::Vertex;

mod vertex;

const VERTICES: &[Vertex] = &[
    Vertex { // vertex a, index = 0
        position: [-0.5, -0.5],
        color:[1.0, 0.0, 0.0]
    },
    Vertex { // vertex b, index = 1
        position: [0.5, -0.5],
        color: [0.0, 1.0, 0.0]
    },
    Vertex { // vertex c, index = 2
        position: [0.5, 0.5],
        color: [0.0, 0.0, 1.0]
    },
    Vertex { // vertex d, index = 3
        position: [-0.5, 0.5],
        color:[1.0, 1.0, 0.0]
    }
];

const INDICES: &[u16] = &[0, 1, 3, 3, 1, 2];

fn main() {
    run_wgpu(
        &WindowConfiguration {
            title: "Ch4. Square (indexed)",
        },
        RenderConfiguration  {
            shader_source: include_str!("triangle.wgsl"),
            vertices: INDICES.len(),
            vertex_buffers: &[VertexBufferInfo::buffer("Vertices", VERTICES)],
            index_buffer: Some(BufferInfo::buffer("Indices", INDICES)),
            ..RenderConfiguration::default()
        },
    )
}
