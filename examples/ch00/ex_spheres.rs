use core::f32::consts::PI;
use core::iter::Peekable;
use std::fs;
use std::num::ParseFloatError;

use anyhow::{Context, Result};
use bytemuck::{Pod, Zeroable};
use cgmath::{ElementWise, Matrix, Matrix3, Matrix4, MetricSpace, Point3, point3, Rad, SquareMatrix, vec3, Vector3};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use winit::event::{DeviceEvent, ElementState};

use webgpu_book::{Configurator, Content, PipelineConfiguration, RenderConfiguration, To, UniformInfo, VertexBufferInfo};
use webgpu_book::transforms::invert;

use crate::common::{CmdArgs, Vertex, VertexN, VertexNC};
use crate::common::light::{LightExamples, LightUniform, Model, OglCamera, UniformsData};
use crate::common::surface_data::{Edges, Mesh, Quads};
use crate::common::vertex_data::sphere_quads;

#[path = "../common/global_common.rs"]
mod common;

struct Sphere {
    center: Point3<f32>,
    radius: f32,
    color: Point3<f32>,
}

impl Sphere {
    #[allow(dead_code)]
    fn quads(&self, n: usize) -> Quads<VertexNC> {
        sphere_quads(self.center, self.radius, n * 2, n, &|position, normal, _uv| {
            VertexNC::new(position, normal, self.color)
        })
    }

    #[allow(dead_code)]
    fn edges(&self, n: usize) -> Edges<VertexN> {
        self.quads(n).into()
    }

    #[allow(dead_code)]
    fn billboards(&self) -> Quads<SphereVertex> {
        let v = [0, 1, 2, 3].map(|i| SphereVertex {
            center: self.center.to_homogeneous().into(),
            color: self.color.to_homogeneous().into(),
            radius: self.radius,
            index: i,
        });
        [v].into_iter().into()
        // [[v[0], v[3], v[2]], [v[2], v[1], v[0]]]
    }

    fn volume(&self) -> f32 {
        4.0 / 3.0 * PI * self.radius.powi(3)
    }
}

#[derive(Copy, Clone, Debug, Pod, Zeroable)]
#[repr(C)]
struct SphereVertex {
    center: [f32; 4],
    color: [f32; 4],
    radius: f32,
    index: u32,
}

impl VertexBufferInfo for SphereVertex {
    const NAME: &'static str = "Sphere";
    const ATTRIBUTES: &'static [wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![0=>Float32x4, 1=>Float32x4, 2=>Float32, 3=>Uint32];
    const ATTRIBUTE_NAMES: &'static [&'static str] = &["center", "color", "radius", "index"];
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct CameraViewProjectUniform {
    view: [[f32; 4]; 4],
    project: [[f32; 4]; 4],
    project_view_inverse: [[f32; 4]; 4],
    eye: [f32; 4],
}

impl UniformInfo for CameraViewProjectUniform {
    const STRUCT_NAME: &'static str = "CameraViewProjectUniform";
    const BINDING_NAME: &'static str = "camera_u";
    const ATTRIBUTES: &'static [(&'static str, &'static str)] = &[
        ("view", "mat4x4<f32>"),
        ("project", "mat4x4<f32>"),
        ("project_view_inverse", "mat4x4<f32>"),
        ("eye", "vec4<f32>")
    ];
}

impl To<CameraViewProjectUniform> for OglCamera {
    fn to(&self) -> CameraViewProjectUniform {
        let view = self.view();
        let project = self.projection();
        // print_matrix("view", view);
        // print_matrix("proj", project);
        // print_matrix("proj * view", project * view);
        let project_view_inverse = invert(project * view);
        // print_matrix("(proj * view)^-1", project_view_inverse);
        CameraViewProjectUniform {
            view: view.into(),
            project: project.into(),
            project_view_inverse: project_view_inverse.into(),
            eye: self.eye().into(),
        }
    }
}

#[allow(clippy::use_debug, dead_code)]
fn print_matrix(name: &str, value: Matrix4<f32>) {
    println!("{name}");
    for r in 0..4 {
        println!("    {:6.3?}", value.row(r));
    }
}

fn light<CU: UniformInfo>() -> Configurator<PipelineConfiguration> where OglCamera: To<CU> {
    let z = 3.0;
    let camera = OglCamera::new(
        point3(0.0, 0.0, z),
        point3(0.0, 0.0, 0.0),
        Vector3::unit_y(),
        Rad((1.0 / z).asin() * 2.0) * 0.0,
    );
    LightExamples::configurator::<1, (), CU>(
        [Model::new(Matrix4::identity())],
        true,
        camera,
        LightUniform::example(),
        (),
        Box::new(Controller::new(Rad(0.005)))
    )
}

#[allow(dead_code)]
fn two_spheres(scale: f32, rng: &mut ChaCha8Rng) -> Vec<Sphere> {
    vec![
        Sphere {
            center: point3(0.5, 0.0, 0.0) * scale,
            radius: 0.5 * scale,
            color: point3(rng.gen(), rng.gen(), rng.gen()),
        },
        Sphere {
            center: point3(-0.5, 0.0, 0.0) * scale,
            radius: 1.0 * scale,
            color: point3(rng.gen(), rng.gen(), rng.gen()),
        },
    ]
}

#[allow(dead_code)]
fn unit_sphere(scale: f32, rng: &mut ChaCha8Rng) -> Vec<Sphere> {
    vec![
        Sphere {
            center: point3(0.0, 0.0, 0.0) * scale,
            radius: scale,
            color: point3(rng.gen(), rng.gen(), rng.gen()),
        },
    ]
}

#[allow(dead_code)]
fn random_spheres(n: usize, scale: f32, rng: &mut ChaCha8Rng) -> Vec<Sphere> {
    const ONES: Vector3<f32> = vec3(1.0, 1.0, 1.0);
    let radius_scale = scale * 2.0 / (n as f32).powf(0.2);
    (0..n)
        .map(|_| {
            Sphere {
                center: (Point3::from(rng.gen::<[f32; 3]>()) * 2.0 - ONES) * scale,
                radius: (0.1 + rng.gen::<f32>()) * radius_scale,
                color: point3(rng.gen(), rng.gen(), rng.gen()),
            }
        })
        .collect()
}


fn main() -> Result<()> {
    const SCALE: f32 = 0.9;
    let rng = &mut ChaCha8Rng::seed_from_u64(12345);

    let spheres = CmdArgs::get_option::<String>("--cif")
        .map_or_else(
            || Ok(CmdArgs::get_option::<usize>("--random").map_or(
                two_spheres(SCALE, rng),
                |n| random_spheres(n, SCALE, rng),
            )),
            |filename| load(filename, SCALE * 1.5),
        )?;
    let edges = CmdArgs::get_option::<usize>("--edges");
    let triangles = CmdArgs::get_option::<usize>("--edges");
    let save_image = CmdArgs::get_option::<String>("--save-image");

    // let n = CmdArgs::next("1000").parse().expect("Invalid number");
    // let spheres = random_spheres(n, SCALE, rng);
    let total: f32 = spheres.iter().map(Sphere::volume).sum();
    println!("Sphere volume: avg = {:.5}, total = {total:.5}", total / spheres.len() as f32);
    // let spheres = two_spheres(SCALE, rng);
    // let spheres = unit_sphere(SCALE, rng);

    let mut render = RenderConfiguration::new();
    if let Some(n) = triangles {
        render.new_pass(vec![pipeline(include_str!("spheres_triangles.wgsl"))
            .with(quads(&spheres, n).cast::<VertexNC>().triangles().vertices())]);
    } else {
        render.new_pass(vec![pipeline(include_str!("spheres.wgsl"))
            .with(Quads::join(spheres.iter().map(Sphere::billboards)).triangles().vertices())
            .with_cull_mode(None)]);
    }
    if let Some(n) = edges {
        render.new_pass(vec![pipeline(include_str!("spheres_edges.wgsl"))
            .with(quads(&spheres, n).cast::<Vertex>().edges().vertices())])
            .with_load(wgpu::LoadOp::Load);
    };
    if let Some(filename)= save_image {
        render.save_images_as(filename.as_str());
    }
    render.run_title("Chapter 0. Spheres");
}

fn quads(spheres: &[Sphere], n: usize) -> Mesh<VertexNC, 4> {
    Quads::join(spheres.iter().map(|sphere| sphere.quads(n)))
}

fn pipeline(shader: &str) -> PipelineConfiguration {
    PipelineConfiguration::new(shader)
        .with(light::<CameraViewProjectUniform>())
}

fn take_while<T, I: Iterator<Item=T>>(peekable: &mut Peekable<I>, p: fn(&T) -> bool) -> Vec<T> {
    let mut result = Vec::new();
    while let Some(item) = peekable.next_if(p) {
        result.push(item);
    }
    result
}


struct Controller {
    speed: Rad<f32>,
    mouse_pressed: bool,
}

impl Controller {
    fn new(speed: Rad<f32>) -> Self {
        Self { speed, mouse_pressed: false }
    }
}

impl Content<&mut UniformsData<1, ()>> for Controller {
    fn input(&mut self, context: &mut UniformsData<1, ()>, event: &DeviceEvent) {
        match *event {
            DeviceEvent::Button { button: 1, state} =>
                self.mouse_pressed = state == ElementState::Pressed,
            DeviceEvent::MouseMotion { delta } => {
                if self.mouse_pressed {
                    context.camera.as_mut().transform(Matrix3::from_angle_y(-self.speed * delta.0 as f32) *
                        Matrix3::from_angle_x(-self.speed * delta.1 as f32));
                }
            }
            _ => (),
        }
    }
}

#[allow(clippy::indexing_slicing, clippy::match_on_vec_items)]
fn load(cif_file: String, scale: f32) -> Result<Vec<Sphere>> {
    let mut atoms: Vec<Sphere> = vec![];
    let mut cell_length = point3(1.0, 1.0, 1.0);

    let file = fs::read_to_string(cif_file).context("Open CIF file")?;
    let lines = &mut file.lines()
        .filter(|line| !line.trim().is_empty() && !line.starts_with('#') && !line.starts_with(';'))
        .peekable();
    while let Some(first) = lines.next() {
        let first_parts = first.split_whitespace().collect::<Vec<_>>();
        if first == "loop_" {
            let columns: Vec<&str> = take_while(lines, |&line| line.starts_with('_'));
            let items: Vec<&str> = take_while(lines, |&line| !line.starts_with('_') && line != "loop_");
            if let Some([atom, x, y, z]) = find_attrs(
                &columns, "_atom_site_",
                ["type_symbol", "fract_x", "fract_y", "fract_z"]
            ) {
                atoms.append(&mut items.iter()
                    .map(|&line| -> Vec<&str> { line.split_whitespace().collect() })
                    .map(|line| {
                        let (radius, color) = match line[atom] {
                            "H" => (120, point3(1.0, 1.0, 1.0)),
                            "O" => (152, point3(0.0, 0.0, 1.0)),
                            "N" => (155, point3(0.0, 0.0, 1.0)),
                            "C" => (170, point3(0.0, 0.0, 0.0)),
                            "S" => (180, point3(1.0, 1.0, 0.0)),
                            "Cu" => (140, point3(1.0, 1.0, 0.0)),
                            "Cl" => (175, point3(0.0, 1.0, 0.0)),
                            "Si" => (210, point3(1.0, 1.0, 0.5)),
                            "Li" => (182, point3(1.0, 0.75, 0.75)),
                            "Rh" => (151, point3(1.0, 0.75, 0.75)),
                            "P" => (180, point3(1.0, 0.4, 0.0)),
                            _ => panic!("Unknown atom {}", line[atom]),
                        };
                        Ok(Sphere {
                            center: point3(
                                parse_float(line[x])?,
                                parse_float(line[y])?,
                                parse_float(line[z])?,
                            ),
                            radius: radius as f32,
                            color,
                        })
                    }).collect::<Result<Vec<Sphere>>>()?);
            }
        } else if first_parts[0] ==  "_cell_length_a" {
            cell_length.x = parse_float(first_parts[1])?;
        } else if first_parts[0] == "_cell_length_b" {
            cell_length.y = parse_float(first_parts[1])?;
        } else if first_parts[0] == "_cell_length_c" {
            cell_length.z = parse_float(first_parts[1])?;
        } else {
            // Do nothing
        }
    }

    for atom in &mut atoms {
        atom.center = atom.center.mul_element_wise(cell_length);
    }

    let min = point3(
        atoms.iter().map(|sphere| sphere.center.x).fold(f32::INFINITY, f32::min),
        atoms.iter().map(|sphere| sphere.center.y).fold(f32::INFINITY, f32::min),
        atoms.iter().map(|sphere| sphere.center.z).fold(f32::INFINITY, f32::min),
    );
    let max = point3(
        atoms.iter().map(|sphere| sphere.center.x).fold(-f32::INFINITY, f32::max),
        atoms.iter().map(|sphere| sphere.center.y).fold(-f32::INFINITY, f32::max),
        atoms.iter().map(|sphere| sphere.center.z).fold(-f32::INFINITY, f32::max),
    );
    let delta = max - min;
    let inner_scale = delta.x.max(delta.y).max(delta.z);
    let center = point3(0.0, 0.0, 0.0);
    let middle = min + (max - min) / 2.0;
    for atom in &mut atoms {
        atom.center = (center - (atom.center - middle) / inner_scale * 2.0) * scale;
    }
    let mut distances = atoms.iter().enumerate().flat_map(|(i, atom1)| atoms[i + 1..].iter()
        .map(|atom2| atom1.center.distance(atom2.center) / (atom1.radius + atom2.radius)))
        .collect::<Vec<_>>();
    distances.sort_by(|a, b| a.partial_cmp(b).expect("always succeed"));
    let q = distances[atoms.len()];
    for atom in &mut atoms {
        atom.radius *= q * 1.5;
    }
    Ok(atoms)
}

fn parse_float(value: &str) -> Result<f32, ParseFloatError> {
    value.split('(').next().expect("always succeed").parse()
}

fn find_attrs<const L: usize>(columns: &[&str], prefix: &str, names: [&str; L]) -> Option<[usize; L]> {
    let prefix_s = prefix.to_owned();
    let options = names.map(|name| columns.iter().position(|&attr| attr == prefix_s.clone() + name));
    let result = options.into_iter().collect::<Option<Vec<usize>>>();
    result.map(|found| found.try_into().expect("always succeed"))
}
