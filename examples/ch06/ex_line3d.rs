use crate::common::{run_example, Vertex};

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
    run_example(
        "Chapter 6 Line",
        include_str!("line3d.wgsl"),
        create_vertices(),
        wgpu::PrimitiveTopology::LineStrip,
        None,
    );
}
