use cgmath::point3;

use crate::common::mvp::AnimationState;
use crate::common::Vertex;
use crate::common::vertex_data::sphere_quads;

mod common;

fn main() {
    let vertex_f = &|position, _normal, _lon_lat| Vertex::new(position);
    sphere_quads(point3(0.0, 0.0, 0.0), 1.7, 20, 20, vertex_f).edges()
        .into_config()
        .with(AnimationState::example())
        .run_title("Chapter 7. Sphere");
}
