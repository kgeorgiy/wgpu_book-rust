use crate::common::light::{ProtoUniforms, TwoSideLightAux};

pub use self::common::*;

#[path = "../common/common.rs"]
mod common;


pub fn proto_example(is_two_side: bool) -> ProtoUniforms<TwoSideLightAux> {
    ProtoUniforms::example_aux(
        include_str!("shader.wgsl").to_owned(),
        None,
        TwoSideLightAux::new(is_two_side)
    )
}

pub fn run_surface(title: &str, vertices: &[VertexNC]) -> ! {
    let is_two_side = CmdArgs::next("false").parse().expect("true of false");
    proto_example(is_two_side).run(title, vertices)
}
