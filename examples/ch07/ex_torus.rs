use webgpu_book::PipelineConfiguration;

use crate::common::mvp::AnimationState;
use crate::common::Vertex;
use crate::common::vertex_data::Torus;

#[allow(clippy::duplicate_mod)]
mod common;

pub fn pipeline() -> PipelineConfiguration {
    Torus::quads(1.5, 0.3, 20, 10).cast::<Vertex>().edges().into_config()
        .with(AnimationState::example())
}

#[allow(dead_code)]
fn main() {
    pipeline().run_title("Chapter 7. Torus");
}
