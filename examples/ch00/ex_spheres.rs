use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, Point3, point3, Rad, SquareMatrix, vec3, Vector3, Zero};
use rand::{Rng, SeedableRng};

use webgpu_book::{PipelineConfiguration, RenderConfiguration, RenderPassConfiguration, To, VertexBufferInfo};
use webgpu_book::boxed::FuncBox;

use crate::common::light::{LightExamples, LightUniforms, Model, OglCamera};
use crate::common::surface_data::Mesh;
use crate::common::vertex_data::{sphere_edges, sphere_faces};
use crate::common::{VertexN, VertexNC};

#[path = "../common/global_common.rs"]
mod common;

struct Sphere {
    center: Point3<f32>,
    radius: f32,
    color: Point3<f32>,
}

impl Sphere {
    #[allow(dead_code)]
    fn face_vertices(&self, n: usize) -> Vec<VertexNC> {
        sphere_faces(self.center, self.radius, n * 2, n, &|position, normal, _uv| {
            VertexNC::new(position, normal, self.color)
        })
    }

    #[allow(dead_code)]
    fn edges_vertices(&self, n: usize) -> Mesh<VertexN> {
        sphere_edges(self.center, self.radius, n * 2, n, &|position, normal, _uv| {
            VertexN::new(position, normal)
        })
    }

    #[allow(dead_code)]
    fn sphere_vertices(&self) -> [[SphereVertex; 3]; 2] {
        let v = [0, 1, 2, 3].map(|i| SphereVertex {
            center: self.center.to_homogeneous().into(),
            color: self.color.to_homogeneous().into(),
            radius: self.radius,
            index: i,
        });
        [[v[0], v[3], v[2]], [v[2], v[1], v[0]]]
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
    const ATTRIBUTES: &'static [wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![0=>Float32x4, 1=>Float32x4, 2=>Float32, 3=>Uint32];
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct CameraViewProjectUniform {
    view: [[f32; 4]; 4],
    project: [[f32; 4]; 4],
}

impl To<CameraViewProjectUniform> for OglCamera {
    fn to(&self) -> CameraViewProjectUniform {
        CameraViewProjectUniform { view: self.view().into(), project: self.projection().into() }
    }
}

fn light<CU: Pod>() -> FuncBox<PipelineConfiguration, PipelineConfiguration> where OglCamera: To<CU>{
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
        LightUniforms::example(()),
        1.0
    )
}


fn main() {
    #![allow(dead_code)]
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(12345);
    const ONES: Vector3<f32> = vec3(1.0, 1.0, 1.0);
    const SCALE: f32 = 1.0;

    let spheres: Vec<Sphere> = (0..1000)
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


    // let faces_pipeline = PipelineConfiguration::new(include_str!("spheres_mesh.wgsl"))
    //     .with(light::<CameraViewProjectUniform>())
    //     .with_cull_mode(Some(wgpu::Face::Back))
    //     .with_vertices(spheres.iter().flat_map(|sphere| sphere.face_vertices(8)).collect());

    // let edges_pipeline = Mesh::join(spheres.iter().map(|sphere| sphere.edges_vertices(8))).into_config()
    //     .with(light::<CameraUniform>());

    let points_pipeline = PipelineConfiguration::new(include_str!("spheres.wgsl"))
        .with(light::<CameraViewProjectUniform>())
        .with_vertices(spheres.iter().flat_map(Sphere::sphere_vertices).flatten().collect());

    RenderConfiguration::new(vec![
        // RenderPassConfiguration::new(vec![faces_pipeline]),
        RenderPassConfiguration::new(vec![points_pipeline]),
        // RenderPassConfiguration::new(vec![edges_pipeline]).with_load(wgpu::LoadOp::Load),
    ])
        .run_title("Chapter 0. Spheres");
}
