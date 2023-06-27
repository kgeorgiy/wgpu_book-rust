use core::f32::consts::PI;

use crate::common::Vertex;

mod common;

fn main() {
    let colors: [[f32; 3]; 6] = [
        [1.0, 0.0, 0.0],
        [1.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 1.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 1.0],
    ];

    let vertices: Vec<Vertex> = colors.into_iter().enumerate()
        .map(|(i, color)| {
            let (sin, cos) = (i as f32 * PI / 3.0).sin_cos();
            Vertex { position: [cos * 0.5, sin * 0.5], color }
        })
        .collect();

    let mut indices = Vec::with_capacity(4 * 3);
    for i in 0..4 {
        indices.push(0);
        indices.push(i + 1);
        indices.push(i + 2);
    }

    common::run_example("Chapter 4. Hexagon (indexed)", &vertices, Some(&indices));
}
