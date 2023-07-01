use cgmath::Deg;
use crate::common::vertex_data::Cylinder;
pub use self::global_common::*;

#[path = "../common/global_common.rs"]
mod global_common;


// Geometry

#[allow(dead_code)]
pub fn cylinder_vertex<T: Into<Deg<f32>>>(r: f32, y: f32, theta: T) -> Vertex {
    let theta1 = theta.into();
    Vertex::new(Cylinder::position(r, y, theta1))
}
