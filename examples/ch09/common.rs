use bytemuck::{Pod, Zeroable};
use cgmath::Point3;

use crate::colormap::Colormap;
use crate::common::ch08_common::ProtoUniforms;
use crate::surface_data::{parametric_surface_data, simple_surface_data, Vertex};

#[path = "../ch08/common.rs"]
mod ch08_common;


// LightAux

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct LightAux {
    is_two_side: i32,
    padding: [u8; 12],
}

impl LightAux {
    fn new(is_two_side: bool) -> Self {
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
    print_range(vertices, "x", 0);
    print_range(vertices, "y", 1);
    print_range(vertices, "z", 2);
    proto_example(is_two_side).run(title, vertices);
}

fn get_args() -> (Colormap, bool) {
    let args: Vec<String> = std::env::args().collect();
    let colormap = Colormap::by_name(if args.len() > 1 { &args[1] } else { "jet" });
    let is_two_side = args.len() > 2 && args[2].parse().expect("true of false");
    (colormap, is_two_side)
}

fn print_range(vertices: &[Vertex], name: &str, index: usize) {
    println!(
        "{} {:?} {:?}",
        name,
        vertices.iter().map(|vertex| vertex.position[index]).min_by(|a, b| a.partial_cmp(b).unwrap()),
        vertices.iter().map(|vertex| vertex.position[index]).max_by(|a, b| a.partial_cmp(b).unwrap())
    );
}
