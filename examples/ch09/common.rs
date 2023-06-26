use cgmath::Point3;

use crate::common::colormap::Colormap;
use crate::common::light::{ProtoUniforms, TwoSideLightAux};
use crate::common::surface_data::{parametric_surface_data, simple_surface_data};

pub use self::common::*;

#[path = "../common/common.rs"]
mod common;

#[path = "surface_data.rs"]
mod surface_data;


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
    let (colormap, is_two_side) = get_args();
    let vertices = simple_surface_data(f, &colormap, min_max_n_x, min_max_n_z, scale);
    run_surface(title, is_two_side, &vertices);
}

#[allow(dead_code)]
pub fn run_parametric_surface(
    title: &str,
    f: &dyn Fn(f32, f32) -> Point3<f32>,
    min_max_n_u: (f32, f32, usize),
    min_max_n_v: (f32, f32, usize),
    scale: (f32, f32, f32),
) -> ! {
    let (colormap, is_two_side) = get_args();
    let vertices = parametric_surface_data(f, &colormap, min_max_n_u, min_max_n_v, scale);
    run_surface(title, is_two_side, &vertices);
}

fn run_surface(title: &str, is_two_side: bool, vertices: &[VertexNC]) -> ! {
    proto_example(is_two_side).run(title, vertices)
}

fn get_args() -> (Colormap, bool) {
    let colormap = Colormap::by_name(CmdArgs::next("jet").as_str());
    let value = CmdArgs::next("false");
    let is_two_side = value.parse().expect("true of false");
    (colormap, is_two_side)
}
