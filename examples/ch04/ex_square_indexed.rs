use core::u16;

use crate::common::{run_example, Vertex};

mod common;

const VERTICES: &[Vertex] = &[
    Vertex {
        // vertex a, index = 0
        position: [-0.5, -0.5],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        // vertex b, index = 1
        position: [0.5, -0.5],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        // vertex c, index = 2
        position: [0.5, 0.5],
        color: [0.0, 0.0, 1.0],
    },
    Vertex {
        // vertex d, index = 3
        position: [-0.5, 0.5],
        color: [1.0, 1.0, 0.0],
    },
];

const INDICES: &[u16] = &[0, 1, 3, 3, 1, 2];

fn main() {
    run_example("Chapter 4. Square (indexed)", VERTICES, Some(INDICES));
}
