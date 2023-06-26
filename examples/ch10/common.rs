use cgmath::Point3;

use webgpu_book::{RenderConfiguration, TextureInfo};

use crate::common::light::{ProtoUniforms, TwoSideLightAux};
use crate::common::surface_data::{parametric_surface_data, simple_surface_data};

pub use self::common::*;

#[path = "../common/common.rs"]
mod common;

#[path = "surface_data.rs"]
mod surface_data;

#[allow(dead_code)]
pub fn proto_example(is_two_side: bool) -> ProtoUniforms<TwoSideLightAux> {
    ProtoUniforms::example_aux(
        include_str!("shader.wgsl").to_owned(),
        None,
        TwoSideLightAux::new(is_two_side)
    )
}

#[allow(dead_code)]
pub fn run_simple_surface(
    title: &str,
    f: &dyn Fn(f32, f32) -> f32,
    min_max_n_x: (f32, f32, usize),
    min_max_n_z: (f32, f32, usize),
    scale: f32
) -> ! {
    run_example(title, &simple_surface_data(f, min_max_n_x, min_max_n_z, scale));
}

#[allow(dead_code)]
pub fn run_parametric_surface(
    title: &str,
    f: &dyn Fn(f32, f32) -> Point3<f32>,
    min_max_n_u: (f32, f32, usize),
    min_max_n_v: (f32, f32, usize),
    scale: (f32, f32, f32),
) -> ! {
    run_example(title, &parametric_surface_data(f, min_max_n_u, min_max_n_v, scale));
}

pub fn run_example(title: &str, vertices: &[VertexNT]) -> ! {
    let texture_file = CmdArgs::next("brick");
    let is_two_side = CmdArgs::next("false").parse().expect("true of false");

    let proto = proto_example(is_two_side);
    let configuration = RenderConfiguration {
        textures: vec![TextureInfo {
            file: format!("examples/ch10/assets/{}.png", texture_file).to_string(),
            u_mode: wgpu::AddressMode::Repeat,
            v_mode: wgpu::AddressMode::Repeat,
        }],
        ..proto.config(
            include_str!("shader.wgsl"),
            wgpu::PrimitiveTopology::TriangleList,
            vertices,
        )
    };
    configuration.run_title(title);
}
