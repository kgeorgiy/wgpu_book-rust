#![allow(clippy::extra_unused_type_parameters)]

use core::f32::consts::PI;
use core::time::Duration;

use bytemuck::{Pod, Zeroable};
use cgmath::{Angle, EuclideanSpace, InnerSpace, Matrix, Matrix4, Point3, point3, Rad, SquareMatrix, Vector3};

use webgpu_book::{BufferInfo, BufferWriter, Configurator, Content, func_box, PipelineConfiguration, To, typed_box, Uniform, UniformArray, VertexBufferInfo};
use webgpu_book::boxed::FuncBox;
use webgpu_book::transforms::{create_projection, create_rotation};

use super::{CmdArgs, VertexN};
use super::surface_data::Mesh;

// Camera

#[derive(Clone)]
pub struct OglCamera {
    eye: Point3<f32>,
    look_at: Vector3<f32>,
    up: Vector3<f32>,
    fovy: Rad<f32>,
    projection: Matrix4<f32>,
}

impl OglCamera {
    #[must_use] pub fn new(eye: Point3<f32>, look_at: Vector3<f32>, up: Vector3<f32>, fovy: Rad<f32>) -> Self {
        Self { eye, look_at, up, fovy, projection: create_projection(1.0, fovy) }
    }

    #[must_use] pub fn view(&self) -> Matrix4<f32> {
        Matrix4::look_to_rh(self.eye, self.look_at, self.up)
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Matrix4<f32> {
        self.projection = create_projection(width as f32 / height as f32, self.fovy);
        self.projection
    }

    pub(crate) fn projection(&self) -> Matrix4<f32> {
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

impl<A: Pod> LightUniforms<A> {
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
    #[must_use]
    pub fn read_args_wireframe<V: VertexBufferInfo + Into<VertexN>>(vertices: Vec<V>) -> FuncBox<PipelineConfiguration, PipelineConfiguration> {
        func_box!(move |config: PipelineConfiguration|
            if CmdArgs::is("wireframe") {
                config.with(Self::wireframe(vertices, 0.1))
            } else {
                config.with_vertices(vertices)
            }
        )
    }

    #[must_use]
    pub fn wireframe<V: VertexBufferInfo + Into<VertexN>>(vertices: Vec<V>, normal_len: f32)
        -> Configurator<PipelineConfiguration>
    {
        let vertices_n = vertices.into_iter().map(Into::into).collect::<Vec<_>>();
        #[allow(clippy::indexing_slicing)]
        let mut mesh: Vec<(VertexN, VertexN)> = vertices_n.chunks_exact(3)
            .flat_map(|face| [(face[0], face[1]), (face[1], face[2]), (face[2], face[0])])
            .collect();
        if normal_len > 0.0 {
            mesh.extend(
                vertices_n.into_iter()
                    .map(|vertex| (vertex, vertex.normal_vertex(normal_len)))
            );
        }

        Mesh::from(mesh.into_iter()).conf()
    }

    #[must_use]
    pub fn models<const ML: usize, LA: Pod>(light_aux: LA, models: [Matrix4<f32>; ML], instances: bool) -> Configurator<PipelineConfiguration> {
        let camera = Self::camera();
        let light = LightUniforms::new(
            point3(1.0, 1.0, 0.0),
            0.1, 1.0, 2.0, 30.0,
            light_aux
        );
        Self::configurator(models.map(Model::new), instances, camera, light, 1.0)
    }

    #[must_use]
    pub fn configurator<const ML: usize, LA: Pod>(
        models: [Model; ML],
        instances: bool,
        camera: OglCamera,
        light: LightUniforms<LA>,
        animation_speed: f32
    ) -> FuncBox<PipelineConfiguration, PipelineConfiguration> {
        let side = camera.look_at.cross(camera.up);
        let fragment = FragmentUniforms::new(
            camera.eye.to_homogeneous().into(),
            (side.normalize() - camera.look_at.normalize() * 2.0).extend(0.0).into()
        );

        func_box!(move |mut conf: PipelineConfiguration| {
            let camera_u: CameraUniform = camera.to();
            let models_u: UniformArray<ModelUniforms, ML> = models.to();

            let camera_buffer = BufferInfo::buffer_format("Camera uniform", &[camera_u.to()], wgpu::ShaderStages::VERTEX);
            let fragment_buffer = BufferInfo::buffer_format("Fragment uniform", &[fragment.to()], wgpu::ShaderStages::FRAGMENT);
            let light_buffer = BufferInfo::buffer_format("Light uniform", &[light.to()], wgpu::ShaderStages::FRAGMENT);

            let factory = func_box!(move |[models_writer, camera_writer, fragment_writer, light_writer]: [BufferWriter; 4]| {
                typed_box!(dyn Content, Uniforms {
                    models:
                        if instances {
                            models_writer.to_instance_array::<Model, ML, ModelUniforms>(models)
                        } else {
                            models_writer.to_binding_array::<Model, ML, ModelUniforms>(models)
                        },
                    camera: camera_writer.to_value::<OglCamera, CameraUniform>(camera),
                    fragment: fragment_writer.to_value::<FragmentUniforms, FragmentUniforms>(fragment),
                    light: light_writer.to_value::<LightUniforms<LA>, LightUniforms<LA>>(light),

                    animation_speed,
                })
            });

            if instances {
                conf = conf.with_instances(ML);
                let model_buffer = BufferInfo::buffer_format("Models uniform", &models_u.as_instances(), wgpu::ShaderStages::VERTEX);
                conf.with_uniforms(
                    [model_buffer, camera_buffer, fragment_buffer, light_buffer],
                    factory,
                )
            } else {
                let model_buffer = BufferInfo::buffer_format("Models uniform", models_u.as_bindings(), wgpu::ShaderStages::VERTEX);
                conf.with_multi_uniforms(
                    [model_buffer, camera_buffer, fragment_buffer, light_buffer],
                    factory,
                    (0..ML).map(|i| [i, 0, 0, 0]).collect()
                )
            }
        })
    }

    #[must_use]
    pub fn camera() -> OglCamera {
        let eye = point3(3.0, 1.5, 3.0);
        let look_direction = -eye.to_vec();
        let up_direction = Vector3::unit_y();
        let fovy = Rad(2.0 * PI / 5.0);
        OglCamera::new(eye, look_direction, up_direction, fovy)
    }

    #[must_use]
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
        let angle = Rad(self.animation_speed * dt.as_secs_f32());
        let rotation = create_rotation([angle.sin(), angle.cos(), 0.0]);
        for model in self.models.as_mut().iter_mut() {
            model.rotation = rotation;
        }

        self.light.as_mut().ambient_intensity = dt.as_secs_f32() / 5.0 % 1.0;
        self.camera.as_mut().eye.z = 3.0 + (dt.as_secs_f32() % 6.0 - 3.0).abs();
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

    #[must_use]
    pub fn example<V: VertexBufferInfo + Into<VertexN>>(shader: &str, vertices: Vec<V>)
        -> PipelineConfiguration
    {
        let is_two_side = CmdArgs::next_bool("Is two side", false);
        PipelineConfiguration::new(shader)
            .with(LightExamples::aux(Self::new(is_two_side)))
            .with_cull_mode(None)
            .with(LightExamples::read_args_wireframe(vertices))
    }
}
