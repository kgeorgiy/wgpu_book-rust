use std::time::Duration;
use cgmath::{Matrix4, Point3, Rad, Vector3};
use wgpu::{PrimitiveTopology, VertexAttribute};
use bytemuck::{Pod, Zeroable};

use webgpu_book::{
    BufferInfo, BufferWriter, Content, RenderConfiguration, run_wgpu, TypedBufferDescriptor,
    VertexBufferInfo, WindowConfiguration,
};
use webgpu_book::transforms::{create_projection, create_rotation, create_view};

pub struct State {
    model_transform: Matrix4<f32>,
    view: Matrix4<f32>,
    fovy: Rad<f32>,
    animation_speed: f32,
    uniform_buffer: BufferWriter<[[f32; 4]; 4]>,
    projection: Matrix4<f32>,
}

impl State {
    pub fn run<V: VertexBufferInfo>(
        title: &str,
        shader_source: &str,
        eye: Point3<f32>,
        look_at: Point3<f32>,
        up: Vector3<f32>,
        fovy: Rad<f32>,
        animation_speed: f32,
        model_transform: Matrix4<f32>,
        vertices: &[V],
        topology: PrimitiveTopology,
        indices: Option<&[u16]>,
    ) {
        run_wgpu(
            &WindowConfiguration { title: format!("Ch6. {}", title).as_str() },
            RenderConfiguration {
                shader_source,
                vertices: indices.map_or(vertices.len(), |indices| indices.len()),
                vertex_buffers: &[V::buffer("Vertices", &vertices)],
                index_buffer: indices.map(|indices| u16::buffer("Indices", indices)),
                topology,
                uniform_buffer: Some(TypedBufferDescriptor::new(
                    "Uniform",
                    &[model_transform.into()],
                    wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                    (),
                )),
                content: Box::new(move |uniform_buffer| {
                    Box::new(State {
                        model_transform,
                        view: create_view(eye, look_at, up),
                        fovy,
                        animation_speed,
                        uniform_buffer,
                        projection: create_projection(1.0, fovy),
                    })
                }),
                ..RenderConfiguration::default()
            },
        );
    }

    fn write(&mut self) {
        let pvm = self.projection * self.view * self.model_transform;
        self.uniform_buffer.write_slice(&[pvm.into()]);
    }
}

impl Content for State {
    fn resize(&mut self, width: u32, height: u32) {
        self.projection = create_projection(width as f32 / height as f32, self.fovy);
        self.write();
    }

    fn update(&mut self, dt: Duration) {
        let angle = self.animation_speed * dt.as_secs_f32();
        self.model_transform = create_rotation([angle.sin(), angle.cos(), 0.0]);
        self.write();
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ColorVertex {
    position: [f32; 4],
    color: [f32; 4],
}

#[allow(dead_code)]
impl ColorVertex {
    fn new(pos: [i8; 3], col: [i8; 3]) -> Self {
        ColorVertex {
            position: [pos[0] as f32, pos[1] as f32, pos[2] as f32, 1.0],
            color: [col[0] as f32, col[1] as f32, col[2] as f32, 1.0],
        }
    }

    pub fn create<const L: usize>(pos: [[i8; 3]; L], col: [[i8; 3]; L]) -> Vec<ColorVertex> {
        let mut data = Vec::with_capacity(L);
        for i in 0..pos.len() {
            data.push(ColorVertex::new(pos[i], col[i]));
        }
        data
    }
}

impl VertexBufferInfo for ColorVertex {
    const ATTRIBUTES: &'static [VertexAttribute] = &wgpu::vertex_attr_array![0=>Float32x4, 1=>Float32x4];
}
