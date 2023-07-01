use webgpu_book::PipelineConfiguration;

use crate::common::Vertex;
use crate::common::mvp::MvpController;

mod common;

fn create_vertices() -> Vec<Vertex> {
    (0..300).map(|i| {
        let t = 0.1 * (i as f32) / 30.0;
        let exp = (-t).exp();
        let (sin, cos) = (30.0 * t).sin_cos();
        Vertex::new([exp * sin, 2.0 * t - 1.0, exp * cos])
    }).collect()
}

fn main() {
    PipelineConfiguration::new(include_str!("line3d.wgsl"))
        .with(MvpController::example(()))
        .with_vertices(create_vertices())
        .with_topology(wgpu::PrimitiveTopology::LineStrip)
        .run_title("Chapter 6 Line");
}
