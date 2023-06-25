use bytemuck::{Pod, Zeroable};
use cgmath::Point3;

use crate::common::colormap::Colormap;
pub use crate::common::common08::ProtoUniforms;
use crate::common::common09::surface_data::{parametric_surface_data, simple_surface_data, Vertex};

#[path = "surface_data.rs"]
mod surface_data;

// LightAux

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct LightAux {
    is_two_side: i32,
    padding: [u8; 12],
}

impl LightAux {
    pub fn new(is_two_side: bool) -> Self {
        Self {
            is_two_side: if is_two_side { 1 } else { 0 },
            padding: [0; 12]
        }
    }
}

pub fn proto_example(is_two_side: bool) -> ProtoUniforms<LightAux> {
    ProtoUniforms::example_aux(
        include_str!("shader.wgsl").to_owned(),
        None,
        LightAux::new(is_two_side)
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

fn run_surface(title: &str, is_two_side: bool, vertices: &[Vertex]) -> ! {
    proto_example(is_two_side).run(title, vertices);
}

fn get_args() -> (Colormap, bool) {
    let args: Vec<String> = std::env::args().collect();
    let colormap = Colormap::by_name(if args.len() > 1 { &args[1] } else { "jet" });
    let is_two_side = args.len() > 2 && args[2].parse().expect("true of false");
    (colormap, is_two_side)
}
