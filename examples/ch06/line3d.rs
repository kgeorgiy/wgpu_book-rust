use std::f32::consts::PI;
use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, Rad, SquareMatrix, Vector3};
use wgpu::{PrimitiveTopology, VertexAttribute};

use webgpu_book::{Context, RenderConfiguration, run_wgpu, VertexBufferInfo, WindowConfiguration};
use webgpu_book::transforms::create_view;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
}

impl VertexBufferInfo for Vertex {
    const ATTRIBUTES: &'static [VertexAttribute] = &wgpu::vertex_attr_array![0=>Float32x3];
}

fn create_vertices() -> [Vertex; 300]{
    let mut vertices = [Vertex { position: [0.0, 0.0, 0.0] }; 300];
    for i in 0..300 {
        let t = 0.1 * (i as f32) / 30.0;
        let x = (-t).exp() * (30.0 * t).sin();
        let z = (-t).exp() * (30.0 * t).cos();
        let y = 2.0 * t - 1.0;
        vertices[i] = Vertex { position: [x, y, z] };
    }
    vertices
}

fn main() {
    let eye = (1.5, 1.0, 3.0).into();
    let look_at = (0.0, 0.0, 0.0).into();
    let up = Vector3::unit_y();

    let model_transform: Matrix4<f32> = Matrix4::identity();
    let view = create_view(eye, look_at, up);

    let vertices = create_vertices();

    run_wgpu(
        &WindowConfiguration {
            title: "Ch6. Lines",
        },
        RenderConfiguration {
            shader_source: include_str!("line3d.wgsl"),
            vertices: vertices.len(),
            vertex_buffers: &[Vertex::buffer("Vertices", &vertices[..])],
            context: Context {
                model_transform,
                view,
                fovy: Rad(2.0 * PI / 5.0),
            },
            topology: PrimitiveTopology::LineStrip,
            ..RenderConfiguration::default()
        },
    )
}
