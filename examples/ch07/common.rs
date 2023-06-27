use cgmath::Deg;
use webgpu_book::PipelineConfiguration;

use crate::common::mvp::{AnimationState, MvpMatrix};
use crate::common::vertex_data::cylinder_position;

pub use self::global_common::*;

#[path = "../common/global_common.rs"]
mod global_common;


// Wireframe

pub struct Wireframe {
    vertices: Vec<Vertex>,
}

#[allow(dead_code)]
impl Wireframe {
    pub fn new(capacity: usize) -> Self {
        Self {  vertices: Vec::with_capacity(capacity * 2) }
    }

    pub(crate) fn into_vec(self) -> Vec<Vertex> {
        self.vertices
    }

    pub(crate) fn add_line(&mut self, from: Vertex, to: Vertex) {
        self.vertices.push(from);
        self.vertices.push(to);
    }

    pub fn add_lines(&mut self, lines: &[(Vertex, Vertex)]) {
        for line in lines {
            self.add_line(line.0, line.1);
        }
    }

    pub fn into_config(self) -> PipelineConfiguration {
        AnimationState::example_config_2::<MvpMatrix, Vertex, u16>(
            include_str!("../ch06/line3d.wgsl"),
            &self.into_vec(),
            wgpu::PrimitiveTopology::LineList,
            None
        )
    }
}

impl From<Vec<(Vertex, Vertex)>> for Wireframe {
    fn from(value: Vec<(Vertex, Vertex)>) -> Self {
        Wireframe { vertices: value.into_iter().flat_map(|(f, s)| [f, s]).collect() }
    }
}


// Geometry

#[allow(dead_code)]
pub fn cylinder_vertex<T: Into<Deg<f32>>>(r: f32, y: f32, theta: T) -> Vertex {
    Vertex::new(cylinder_position(r, y, theta.into()))
}
