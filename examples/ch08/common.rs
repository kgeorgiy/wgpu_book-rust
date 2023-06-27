use bytemuck::{Pod, Zeroable};
use cgmath::point3;

use crate::common::light::ProtoUniforms;

pub use self::global_common::*;

#[path = "../common/global_common.rs"]
mod global_common;

// LightAux

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct LightAux {
    color: [f32; 4],
}

impl LightAux {
    #[allow(dead_code)]
    pub fn example() -> ProtoUniforms<LightAux> {
        ProtoUniforms::example_aux(
            include_str!("shader.wgsl").to_owned(),
            None,
            LightAux { color: point3(1.0, 0.0, 0.0).to_homogeneous().into() },
        )
    }
}
