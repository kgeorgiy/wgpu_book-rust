use cgmath::Deg;

use crate::common::vertex_data::cylinder_position;

pub use self::common::*;

#[path = "../common/common.rs"]
mod common;


// Wireframe

pub struct Wireframe {
    vertices: Vec<Vertex>,
}

#[allow(dead_code)]
impl Wireframe {
    pub fn new(capacity: usize) -> Self {
        Self {  vertices: Vec::with_capacity(capacity * 2) }
    }

    pub(crate) fn to_vec(self) -> Vec<Vertex> {
        self.vertices
    }

    pub(crate) fn add_line(&mut self, from: Vertex, to: Vertex) {
        self.vertices.push(from);
        self.vertices.push(to)
    }

    pub fn add_lines(&mut self, lines: &[(Vertex, Vertex)]) {
        for line in lines {
            self.add_line(line.0, line.1)
        }
    }

    pub fn show(self, title: &str) {
        AnimationState::run::<MvpMatrix, Vertex, u16>(
            title,
            include_str!("../ch06/line3d.wgsl"),
            1.0,
            &self.to_vec(),
            wgpu::PrimitiveTopology::LineList,
            None,
        );
    }
}

impl From<Vec<(Vertex, Vertex)>> for Wireframe {
    fn from(value: Vec<(Vertex, Vertex)>) -> Self {
        Wireframe { vertices: value.iter().flat_map(|(f, s)| [*f, *s]).collect() }
    }
}


// Geometry

#[allow(dead_code)]
pub fn cylinder_vertex<T: Into<Deg<f32>>>(r: f32, y: f32, theta: T) -> Vertex {
    Vertex::new(cylinder_position(r, y, theta.into()))
}
