use crate::common::{Vertex};
use crate::common::mvp::AnimationState;
use crate::common::vertex_data::Cylinder;

mod common;

fn main() {
    Cylinder::quads(0.5, 1.0, 2.5, 20, 0.0, 0.0)
        .cast::<Vertex>()
        .edges().into_config()
        .with(AnimationState::example())
        .run_title("Chapter 7. Cylinder");
}
