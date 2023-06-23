use webgpu_book::{BufferInfo, RenderConfiguration, run_wgpu, WindowConfiguration};

use crate::vertex::Vertex;

mod vertex;

const VERTICES: [Vertex; 3] = [
    Vertex {
        position: [0.0, 0.5],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5],
        color: [0.0, 0.0, 1.0],
    },
];

fn main() {
    run_wgpu(
        &WindowConfiguration {
            title: "Ch4. Triangle",
        },
        RenderConfiguration  {
            shader_source: include_str!("triangle.wgsl"),
            vertices: VERTICES.len(),
            vertex_buffers: &[Vertex::buffer("Vertices", &VERTICES)],
            ..RenderConfiguration::default()
        },
    )
}
