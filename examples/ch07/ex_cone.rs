use common::surface_data::Edges;

use crate::common::mvp::AnimationState;
use crate::common::Vertex;
use crate::common::vertex_data::Cone;

mod common;

fn main() {
    Edges::from(Cone::triangles(0.5, 1.5, 2.0, 12).cast::<Vertex>()).into_config()
        .with(AnimationState::example())
        .run_title("Chapter 7. Cone");
}
