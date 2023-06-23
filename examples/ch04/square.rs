use webgpu_book::{BufferInfo, RenderConfiguration, run_wgpu, WindowConfiguration};

use crate::vertex::Vertex;

mod vertex;

const VERTICES: &[Vertex] = &[
    Vertex { // vertex a
        position: [-0.5, -0.5],
        color:[1.0, 0.0, 0.0]
    },
    Vertex { // vertex b
        position: [0.5, -0.5],
        color: [0.0, 1.0, 0.0]
    },
    Vertex { // vertex d
        position: [-0.5, 0.5],
        color:[1.0, 1.0, 0.0]
    },
    Vertex { // vertex d
        position: [-0.5, 0.5],
        color:[1.0, 1.0, 0.0]
    },
    Vertex { // vertex b
        position: [0.5, -0.5],
        color: [0.0, 1.0, 0.0]
    },
    Vertex { // vertex c
        position: [0.5, 0.5],
        color: [0.0, 0.0, 1.0]
    },
];

fn main() {
    run_wgpu(
        &WindowConfiguration {
            title: "Ch4. Square",
        },
        RenderConfiguration  {
            shader_source: include_str!("triangle.wgsl"),
            vertices: VERTICES.len(),
            vertex_buffers: &[Vertex::buffer("Vertices", VERTICES)],
            ..RenderConfiguration::default()
        },
    )
}
