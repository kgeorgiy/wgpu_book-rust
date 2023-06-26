use crate::common::{run_example, Vertex};

mod common;

const VERTICES: [Vertex; 3] = [
    Vertex {
        position: [0.0, 0.5],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5],
        color: [0.0, 0.0, 1.0],
    },
];

fn main() {
    run_example("Chapter 4. Triangle", &VERTICES, None);
}
