use std::f32::consts::PI;

use crate::common04::Vertex;

mod common04;

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

    common04::run_example("Ch4. Hexagon (indexed)", &vertices, Some(&indices));
}
