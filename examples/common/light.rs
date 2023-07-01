#![allow(clippy::extra_unused_type_parameters)]

use core::time::Duration;

use bytemuck::{Pod, Zeroable};
use cgmath::{Angle, InnerSpace, Matrix, Matrix4, Point3, point3, Rad, SquareMatrix, Vector3, Zero};

use webgpu_book::{Configurator, Content, func_box, PipelineConfiguration, To, Uniform, VertexBufferInfo};
use webgpu_book::boxed::FuncBox;
use webgpu_book::transforms::{create_projection, create_rotation};

use super::{CmdArgs, VertexN};
use super::surface_data::Edges;
use super::surface_data::Triangles;

// Camera

#[derive(Clone)]
pub struct OglCamera {
    eye: Point3<f32>,
    look_at: Point3<f32>,
    up: Vector3<f32>,
    fovy: Rad<f32>,
    projection: Matrix4<f32>,
}

impl OglCamera {
    #[must_use]
    pub fn new(eye: Point3<f32>, look_at: Point3<f32>, up: Vector3<f32>, fovy: Rad<f32>) -> Self {
        Self { eye, look_at, up, fovy, projection: create_projection(1.0, fovy) }
    }

    #[must_use]
    pub fn view(&self) -> Matrix4<f32> {
        Matrix4::look_to_rh(self.eye, self.look_at - self.eye, self.up)
    }

    #[must_use]
    pub fn projection(&self) -> Matrix4<f32> {
        self.projection
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Matrix4<f32> {
        self.projection = create_projection(width as f32 / height as f32, self.fovy);
        self.projection
    }
}


// FragmentUniforms

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct FragmentUniforms {
    light_position: [f32; 4],
    eye_position: [f32; 4],
}

impl FragmentUniforms {
    #[must_use] pub fn new(eye: [f32; 4], light: [f32; 4]) -> Self {
        Self { eye_position: eye, light_position: light }
    }
}


// Light

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct LightUniforms<A> {
    specular_color: [f32; 4],
    pub(crate) ambient_intensity: f32,
    pub(crate) diffuse_intensity: f32,
    pub(crate) specular_intensity: f32,
    pub(crate) specular_shininess: f32,
    pub(crate) aux: A,
}

impl<A> LightUniforms<A> {
    pub fn new(
        specular_color: Point3<f32>,
        ambient_intensity: f32,
        diffuse_intensity: f32,
        specular_intensity: f32,
        specular_shininess: f32,
        aux: A,
    ) -> Self {
        Self {
            specular_color: specular_color.to_homogeneous().into(),
            ambient_intensity,
            diffuse_intensity,
            specular_intensity,
            specular_shininess,
            aux,
        }
    }

    pub fn example(aux: A) -> LightUniforms<A> {
        LightUniforms::new(
            point3(1.0, 1.0, 0.0),
            0.1, 1.0, 2.0, 30.0,
            aux
        )
    }
}



// Model, ModelUniforms

#[derive(Clone, Debug)]
pub struct Model {
    model: Matrix4<f32>,
    rotation: Matrix4<f32>,
}

impl Model {
    #[must_use] pub fn new(model: Matrix4<f32>) -> Self {
        Self { model, rotation: Matrix4::identity() }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ModelUniforms {
    points: [[f32; 4]; 4],
    vectors: [[f32; 4]; 4],
}

impl To<ModelUniforms> for Model {
    fn to(&self) -> ModelUniforms {
        let model = self.model * self.rotation;
        ModelUniforms {
            vectors: model.invert().expect("invertible matrix").transpose().into(),
            points: model.into(),
        }
    }
}


// View, ViewUniforms

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct CameraUniform {
    view_project: [[f32; 4]; 4],
}

impl To<CameraUniform> for OglCamera {
    fn to(&self) -> CameraUniform {
        CameraUniform { view_project: (self.projection() * self.view()).into() }
    }
}

// ProtoUniforms

pub struct LightExamples;

impl LightExamples {
    pub fn read_args_wireframe<V: VertexBufferInfo + Into<VertexN>>(triangles: Triangles<V>) -> FuncBox<PipelineConfiguration, PipelineConfiguration> {
        func_box!(move |config: PipelineConfiguration|
            if CmdArgs::is("wireframe") {
                config.with(Self::wireframe(triangles, 0.1))
            } else {
                config.with_vertices(triangles.vertices())
            }
        )
    }

    pub fn wireframe<V: VertexBufferInfo + Into<VertexN>>(triangles: Triangles<V>, normal_len: f32)
        -> Configurator<PipelineConfiguration>
    {
        let mapped: Triangles<VertexN> = triangles.cast();
        #[allow(clippy::indexing_slicing)]
        let mut edges: Vec<[VertexN; 2]> = mapped.iter()
            .flat_map(|tri| [[tri[0], tri[1]], [tri[1], tri[2]], [tri[2], tri[0]]])
            .collect();
        if normal_len > 0.0 {
            edges.extend(
                mapped.vertices().iter()
                    .map(|vertex| [*vertex, vertex.normal_vertex(normal_len)])
            );
        }

        Edges::from(edges.into_iter())
            .conf_shader(include_str!("wireframe.wgsl"))
    }

    pub fn models<const ML: usize, LA: Pod>(light_aux: LA, models: [Matrix4<f32>; ML], instances: bool) -> Configurator<PipelineConfiguration> {
        let camera = OglCamera::new(
            point3(3.0, 1.5, 3.0),
            point3(0.0, 0.0, 0.0),
            Vector3::unit_y(),
            Rad::full_turn() / 5.0,
        );
        Self::configurator::<ML, LA, CameraUniform>(
            models.map(Model::new),
            instances,
            camera,
            LightUniforms::example(light_aux),
            1.0
        )
    }

    pub fn configurator<const ML: usize, LA: Pod, CU: Pod>(
        models: [Model; ML],
        instances: bool,
        camera: OglCamera,
        light: LightUniforms<LA>,
        animation_speed: f32
    ) -> FuncBox<PipelineConfiguration, PipelineConfiguration> where OglCamera: To<CU> {
        func_box!(move |pipeline: PipelineConfiguration| {
            Self::configure::<ML, LA, CU>(pipeline, models, instances, camera, light, animation_speed)
        })
    }

    fn configure<const ML: usize, LA: Pod, CU: Pod>(
        mut pipeline: PipelineConfiguration,
        models: [Model; ML],
        instances: bool,
        camera: OglCamera,
        light: LightUniforms<LA>,
        animation_speed: f32,
    ) -> PipelineConfiguration where OglCamera: To<CU> {
        let forward = (camera.look_at - camera.eye).normalize();
        let side = forward.cross(camera.up).normalize();
        let fragment = FragmentUniforms::new(
            camera.eye.to_homogeneous().into(),
            (side - forward * 2.0).extend(0.0).into()
        );

        let uniforms = pipeline.uniforms();

        let unif = Uniforms {
            models: (
                if instances {
                    uniforms
                        .instances(ML)
                        .add("Models", models, wgpu::ShaderStages::VERTEX)
                        .instance_array::<ModelUniforms>()
                } else {
                    uniforms
                        .variants((0..ML).map(|i| vec![i]).collect())
                        .add("Models", models, wgpu::ShaderStages::VERTEX)
                        .bindings_array::<ModelUniforms>()
                }
            ),
            camera: uniforms.add("Camera", camera, wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT)
                .value::<CU>(),
            fragment: uniforms.add("Fragment", fragment, wgpu::ShaderStages::FRAGMENT)
                .value::<FragmentUniforms>(),
            light: uniforms.add("Light", light, wgpu::ShaderStages::FRAGMENT)
                .value::<LightUniforms<LA>>(),

            animation_speed,
        };
        pipeline.add_listener(Box::new(unif));
        pipeline
    }

    pub fn aux<LA: Pod>(light_aux: LA) -> Configurator<PipelineConfiguration> {
        Self::models(light_aux, [Matrix4::identity()], true)
    }
}

// Uniforms

#[allow(dead_code)]
pub struct Uniforms<const ML: usize, LA: Pod> {
    models: Uniform<[Model; ML]>,
    camera: Uniform<OglCamera>,
    fragment: Uniform<FragmentUniforms>,
    light: Uniform<LightUniforms<LA>>,

    animation_speed: f32,
}

impl<const ML: usize, LA: Pod> Content for Uniforms<ML, LA> {
    fn resize(&mut self, width: u32, height: u32) {
        self.camera.as_mut().resize(width, height);
    }

    fn update(&mut self, dt: Duration) {
        let time = self.animation_speed * dt.as_secs_f32();
        let (angle_sin, angle_cos) = (Rad::full_turn() * time / 5.0).sin_cos();
        let rotation = create_rotation([
            Rad::full_turn() * angle_sin / 2.0,
            Rad::full_turn() * angle_cos / 2.0,
            Rad::zero()
        ]);

        for model in self.models.as_mut().iter_mut() {
            model.rotation = rotation;
        }

        self.light.as_mut().ambient_intensity = Self::saw(time / 4.0);
        self.camera.as_mut().eye.z = 3.0 + Self::saw(time / 6.0) * 10.0;
    }
}

impl<LA: Pod, const ML: usize> Uniforms<ML, LA> {
    fn saw(time: f32) -> f32 {
        (time % 1.0 - 0.5).abs() * 2.0
    }
}


// TwoSideLightAux

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct TwoSideLightAux {
    is_two_side: i32,
    padding: [u8; 12],
}

impl TwoSideLightAux {
    #[must_use]
    pub fn new(is_two_side: bool) -> Self {
        Self {
            is_two_side: i32::from(is_two_side),
            padding: [0; 12]
        }
    }

    pub fn example<V: VertexBufferInfo + Into<VertexN>>(shader: &str, triangles: Triangles<V>)
        -> PipelineConfiguration
    {
        PipelineConfiguration::new(shader)
            .with(Self::read_args())
            .with_cull_mode(None)
            .with(LightExamples::read_args_wireframe(triangles))
    }

    pub fn read_args() -> Configurator<PipelineConfiguration> {
        LightExamples::aux(Self::new(CmdArgs::next_bool("Is two side", false)))
    }
}
