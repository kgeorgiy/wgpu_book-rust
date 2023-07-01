use cgmath::point3;

use common::surface_data::Mesh;

use crate::common::mvp::AnimationState;
use crate::common::Vertex;
use crate::common::vertex_data::sphere_edges;

mod common;

fn create_mesh(r: f32, u: usize, v: usize) -> Mesh<Vertex> {
    sphere_edges(point3(0.0, 0.0, 0.0), r, u, v, &|position, _normal, _lon_lat| Vertex::new(position))
}

fn main() {
    create_mesh(1.7, 20, 20).into_config()
        .with(AnimationState::example())
        .run_title("Chapter 7. Sphere");
}
