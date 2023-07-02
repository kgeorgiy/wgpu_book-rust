#![allow(clippy::used_underscore_binding)]

use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, Point3, point3, Rad, SquareMatrix, vec3, Vector3, Zero};
use rand::{Rng, SeedableRng};

use webgpu_book::{Configurator, PipelineConfiguration, RenderConfiguration, RenderPassConfiguration, To, UniformInfo, VertexBufferInfo};

use crate::common::{VertexN, VertexNC};
use crate::common::light::{MergedVPUniform, LightExamples, LightUniform, Model, OglCamera};
use crate::common::surface_data::{Edges, Quads};
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
}

impl UniformInfo for CameraViewProjectUniform {
    const STRUCT_NAME: &'static str = "CameraViewProjectUniform";
    const BINDING_NAME: &'static str = "camera_u";
    const ATTRIBUTES: &'static [(&'static str, &'static str)] = &[
        ("view", "mat4x4<f32>"),
        ("project", "mat4x4<f32>"),
    ];
}

impl To<CameraViewProjectUniform> for OglCamera {
    fn to(&self) -> CameraViewProjectUniform {
        CameraViewProjectUniform { view: self.view().into(), project: self.projection().into() }
    }
}

fn light<CU: UniformInfo>() -> Configurator<PipelineConfiguration> where OglCamera: To<CU>{
    let camera = OglCamera::new(
        point3(3.0, 1.5, 3.0),
        point3(0.0, 0.0, 0.0),
        Vector3::unit_y(),
        // Rad::full_turn() / 5.0,
        Rad::zero(),
    );
    LightExamples::configurator::<1, (), CU>(
        [Model::new(Matrix4::identity())],
        true,
        camera.clone(),
        LightUniform::example(()),
        1.0
    )
}


fn main() {
    const ONES: Vector3<f32> = vec3(1.0, 1.0, 1.0);
    const SCALE: f32 = 1.0;

    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(12345);

    let spheres: Vec<Sphere> = (0..100)
        .map(|_| {
            Sphere {
                center: (point3(rng.gen(), rng.gen(), rng.gen()) * 2.0 - ONES) * SCALE,
                radius: rng.gen::<f32>() / 3.0 * SCALE,
                color: point3(rng.gen(), rng.gen(), rng.gen()),
            }
        })
        .collect();

    // let spheres: Vec<Sphere> = vec![
    //     Sphere {
    //         center: point3(0.5, 0.0, 0.0) * SCALE,
    //         radius: 0.75 * SCALE,
    //         color: point3(rng.gen(), rng.gen(), rng.gen()),
    //     },
    //     Sphere {
    //         center: point3(-0.5, 0.0, 0.0) * SCALE,
    //         radius: 1.0 * SCALE,
    //         color: point3(rng.gen(), rng.gen(), rng.gen()),
    //     },
    // ];


    let quads = Quads::join(spheres.iter().map(|sphere| sphere.quads(8)));

    let _faces_pipeline = PipelineConfiguration::new(include_str!("spheres_mesh.wgsl"))
        .with(light::<CameraViewProjectUniform>())
        .with_cull_mode(Some(wgpu::Face::Back))
        .with(quads.clone().triangles().vertices());

    let _edges_pipeline = PipelineConfiguration::new(include_str!("../common/wireframe.wgsl"))
        .with(quads.cast::<VertexN>().edges().vertices())
        .with(light::<MergedVPUniform>());

    let _points_pipeline = PipelineConfiguration::new(include_str!("spheres.wgsl"))
        .with(light::<CameraViewProjectUniform>())
        .with(Quads::join(spheres.iter().map(Sphere::billboards)).triangles().vertices());

    RenderConfiguration::new(vec![
        RenderPassConfiguration::new(vec![_faces_pipeline]),
        // RenderPassConfiguration::new(vec![_points_pipeline]),
        // RenderPassConfiguration::new(vec![_edges_pipeline]).with_load(wgpu::LoadOp::Load),
    ])
        .run_title("Chapter 0. Spheres");
}
