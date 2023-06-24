use bytemuck::{Pod, Zeroable};
use wgpu::{PrimitiveTopology, VertexAttribute};
use cgmath::{Angle, Deg};

use webgpu_book::VertexBufferInfo;

use crate::state::AnimationState;

// Vertex

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub(crate) struct Vertex {
    pub position: [f32; 3],
}

impl Vertex {
    pub fn new<P: Into<[f32;3]>>(position: P) -> Self {
        Self { position: position.into() }
    }
}

impl VertexBufferInfo for Vertex {
    const ATTRIBUTES: &'static [VertexAttribute] = &wgpu::vertex_attr_array![0=>Float32x3];
}

// Wireframe

pub struct Wireframe {
    points: Vec<Vertex>,
}

#[allow(dead_code)]
impl Wireframe {
    pub fn new(capacity: usize) -> Self {
        Self {  points: Vec::with_capacity(capacity * 2) }
    }

    pub(crate) fn to_vec(self) -> Vec<Vertex> {
        self.points
    }

    pub(crate) fn add_line(&mut self, from: Vertex, to: Vertex) {
        self.points.push(from);
        self.points.push(to)
    }

    pub(crate) fn add_lines(&mut self, lines: &[(Vertex, Vertex)]) {
        for line in lines {
            self.add_line(line.0, line.1)
        }
    }

    pub(crate) fn show(self, title: &str) {
        AnimationState::run(
            title,
            include_str!("../ch06/line3d.wgsl"),
            1.0,
            &self.to_vec(),
            PrimitiveTopology::LineList,
            None,
        );
    }
}


// Geometry

#[allow(dead_code)]
pub(crate) fn cylinder_position<T: Into<Deg<f32>>>(r: f32, y: f32, theta: T) -> [f32; 3] {
    let (sin_theta, cos_theta) = theta.into().sin_cos();
    [r * cos_theta, y, -r * sin_theta]
}

#[allow(dead_code)]
pub(crate) fn cylinder_vertex<T: Into<Deg<f32>>>(r: f32, y: f32, theta: T) -> Vertex {
    Vertex::new(cylinder_position(r, y, theta))
}
