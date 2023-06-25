use bytemuck::{Pod, Zeroable};
use cgmath::{InnerSpace, Point2, Point3, Vector3};

use surface_data::{parametric_surface_data, simple_surface_data};
use webgpu_book::{RenderConfiguration, TextureInfo, VertexBufferInfo};

use crate::common::common08::run_title;
use crate::common::common09::{LightAux, ProtoUniforms};

mod surface_data;

// Vertex

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 4],
    pub normal: [f32; 4],
    pub uv: [f32; 2],
}

impl Vertex {
    #[allow(dead_code)]
    pub fn new(position: Point3<f32>, normal: Vector3<f32>, uv: Point2<f32>) -> Self {
        Self {
            position: position.to_homogeneous().into(),
            normal: normal.normalize().extend(0.0).into(),
            uv: uv.into(),
        }
    }
}

impl VertexBufferInfo for Vertex {
    const ATTRIBUTES: &'static [wgpu::VertexAttribute] = &wgpu::vertex_attr_array![0=>Float32x4, 1=>Float32x4, 2=>Float32x2];
}

#[allow(dead_code)]
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
    run_surface(title, &simple_surface_data(f, min_max_n_x, min_max_n_z, scale));
}

#[allow(dead_code)]
pub fn run_parametric_surface(
    title: &str,
    f: &dyn Fn(f32, f32) -> Point3<f32>,
    min_max_n_u: (f32, f32, usize),
    min_max_n_v: (f32, f32, usize),
    scale: (f32, f32, f32),
) -> ! {
    run_surface(title, &parametric_surface_data(f, min_max_n_u, min_max_n_v, scale));
}

fn run_surface(title: &str, vertices: &[Vertex]) -> ! {
    run_example(title, vertices);
}

pub fn run_example(title: &str, vertices: &[Vertex]) -> ! {
    let args: Vec<String> = std::env::args().collect();
    let texture_file = if args.len() > 1 { &args[1] } else { "multiple" };
    let is_two_side = args.len() > 2 && args[2].parse().expect("true of false");

    let proto = proto_example(is_two_side);
    run_title(title, RenderConfiguration {
        textures: vec![TextureInfo {
            file: format!("examples/ch10/assets/{}.png", texture_file).to_string(),
            u_mode: wgpu::AddressMode::Repeat,
            v_mode: wgpu::AddressMode::Repeat,
        }],
        ..proto.config(
            include_str!("shader.wgsl").to_string(),
            wgpu::PrimitiveTopology::TriangleList,
            vertices,
        )
    });
}
