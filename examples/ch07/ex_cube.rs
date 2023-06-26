use crate::common::{Vertex, Wireframe};

mod common;

fn create_mesh() -> Wireframe {
    let positions: [[f32; 3]; 8] = [
        [-1.0,  1.0,  1.0],
        [-1.0,  1.0, -1.0],
        [ 1.0,  1.0, -1.0],
        [ 1.0,  1.0,  1.0],
        [-1.0, -1.0,  1.0],
        [-1.0, -1.0, -1.0],
        [ 1.0, -1.0, -1.0],
        [ 1.0, -1.0,  1.0],
    ];
    // line segments
    let lines: [(usize, usize); 12] = [
        // 4 lines on top face
        (0, 1), (1, 2), (2, 3), (3, 0),
        // 4 lines on bottom face
        (4, 5), (5, 6), (6, 7), (7, 4),
        // 4 lines on sides
        (0, 4), (1, 5), (2, 6), (3, 7),
    ];
    Wireframe::from(lines.iter()
        .map(|(f, t)| (Vertex::new(positions[*f]), Vertex::new(positions[*t])))
        .collect::<Vec<_>>())
}

fn main() {
    create_mesh().show("Chapter 7. Cube");
}
