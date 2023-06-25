use std::f32::consts::PI;

use webgpu_book::{BufferInfo, RenderConfiguration, run_wgpu, WindowConfiguration};

use crate::vertex::Vertex;

mod vertex;

fn main() {
    let colors: [[f32; 3]; 6] = [
        [1.0, 0.0, 0.0],
        [1.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 1.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 1.0],
    ];

    let mut vertices = Vec::with_capacity(6);
    for i in 0..6 {
        let (sin, cos) = (i as f32 * PI / 3.0).sin_cos();
        vertices.push(Vertex {
            position: [cos * 0.5, sin * 0.5],
            color: colors[i],
        });
    }

    let mut indices = Vec::with_capacity(5 * 3);
    for i in 0..4 {
        indices.push(0);
        indices.push(i + 1);
        indices.push(i + 2);
    }

    run_wgpu(
        &WindowConfiguration {
            title: "Ch4. Hexagon (indexed)",
        },
        RenderConfiguration {
            shader_source: include_str!("triangle.wgsl"),
            vertices: indices.len(),
            vertex_buffers: &[Vertex::buffer("Vertices", &vertices[..])],
            index_buffer: Some(u16::buffer("Indices", &indices)),
            ..RenderConfiguration::default()
        },
    );
}
