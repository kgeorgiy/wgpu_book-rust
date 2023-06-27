use crate::common::light::TwoSideLightAux;

pub use self::global_common::*;

#[path = "../common/global_common.rs"]
mod global_common;

pub fn run_surface(title: &str, vertices: &[VertexNC]) -> ! {
    TwoSideLightAux::example(include_str!("shader.wgsl"))
        .run(title, vertices)
}
