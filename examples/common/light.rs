#![allow(clippy::extra_unused_type_parameters, clippy::module_name_repetitions)]

use core::f32::consts::PI;
use core::time::Duration;

use bytemuck::{Pod, Zeroable};
use cgmath::{Angle, EuclideanSpace, InnerSpace, Matrix, Matrix4, Point3, point3, Rad, SquareMatrix, Vector3};

use webgpu_book::{BufferInfo, BufferWriter, Content, ContentFactory, PipelineConfiguration, To, Uniform, UniformArray, VertexBufferInfo};
use webgpu_book::transforms::{create_projection, create_rotation};

use super::{CmdArgs, VertexN};


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

pub struct ProtoUniforms<const ML: usize, LA: Pod> {
    models: [Model; ML],
    instances: bool,
    camera: OglCamera,
    fragment: FragmentUniforms,
    light: LightUniforms<LA>,
    animation_speed: f32,
}

impl<const ML: usize, LA: Pod> ProtoUniforms<ML, LA> {
    fn wireframe(config: PipelineConfiguration, vertices: &[VertexN], normal_len: f32) -> PipelineConfiguration {
        let mut wireframe_vertices: Vec<VertexN> = Vec::with_capacity(vertices.len() * 4);
        for face in vertices.chunks_exact(3) {
            #[allow(clippy::indexing_slicing)]
            wireframe_vertices.extend_from_slice(&[face[0], face[1], face[1], face[2], face[2], face[0]]);
        }
        if normal_len > 0.0 {
            for vertex in vertices {
                wireframe_vertices.extend_from_slice(&[*vertex, vertex.normal_vertex(normal_len)]);
            }
        }

        config
            .with_vertices(&wireframe_vertices)
            .with_shader(include_str!("wireframe.wgsl"))
            .with_topology(wgpu::PrimitiveTopology::LineList)
    }

    pub fn example_models<V: VertexBufferInfo + Into<VertexN>>(
        shader_source: &str,
        vertices: &[V],
        light_aux: LA,
        models: [Matrix4<f32>; ML],
        instances: bool
    ) -> PipelineConfiguration {
        let eye = point3(3.0, 1.5, 3.0);
        let look_direction = -eye.to_vec();
        let up_direction = Vector3::unit_y();
        let fovy = Rad(2.0 * PI / 5.0);

        let side = look_direction.cross(up_direction);
        let uniforms = ProtoUniforms {
            models: models.map(Model::new),
            instances,
            camera: OglCamera::new(eye, look_direction, up_direction, fovy),
            fragment: FragmentUniforms::new(
                eye.to_homogeneous().into(),
                (side.normalize() - look_direction.normalize() * 2.0).extend(0.0).into()
            ),
            light: LightUniforms::new(point3(1.0, 1.0, 0.0), 0.1, 1.0, 2.0, 30.0, light_aux),
            animation_speed: 1.0
        };
        let config = uniforms.into_config(shader_source)
            .with_cull_mode(None);
        if CmdArgs::is("wireframe") {
            let vertices_n = vertices.iter().map(|v| (*v).into()).collect::<Vec<_>>();

            Self::wireframe(config, &vertices_n, 0.1)
        } else {
            config.with_vertices(vertices)
        }
    }

    fn into_config(self, shader_source: &str) -> PipelineConfiguration {
        let camera: CameraUniform = self.camera.to();
        let models: UniformArray<ModelUniforms, ML> = self.models.to();

        let camera_buffer = BufferInfo::buffer_format("Camera uniform", &[camera.to()], wgpu::ShaderStages::VERTEX);
        let fragment_buffer = BufferInfo::buffer_format("Fragment uniform", &[self.fragment.to()], wgpu::ShaderStages::FRAGMENT);
        let light_buffer = BufferInfo::buffer_format("Light uniform", &[self.light.to()], wgpu::ShaderStages::FRAGMENT);

        let mut config = PipelineConfiguration::new(shader_source);

        if self.instances {
            config = config.with_instances(ML);
            let model_buffer = BufferInfo::buffer_format("Models uniform", &models.as_instances(), wgpu::ShaderStages::VERTEX);
            config.with_uniforms(
                [model_buffer, camera_buffer, fragment_buffer, light_buffer],
                Box::new(self),
            )
        } else {
            let model_buffer = BufferInfo::buffer_format("Models uniform", models.as_bindings(), wgpu::ShaderStages::VERTEX);
            config.with_multi_uniforms(
                [model_buffer, camera_buffer, fragment_buffer, light_buffer],
                Box::new(self),
                (0..ML).map(|i| [i, 0, 0, 0,]).collect()
            )
        }
    }
}

impl<LA: Pod> ProtoUniforms<1, LA> {
    pub fn example_aux<V: VertexBufferInfo + Into<VertexN>>(
        shader_source: &str,
        vertices: &[V],
        light_aux: LA,
    ) -> PipelineConfiguration {
        Self::example_models(shader_source, vertices, light_aux, [Matrix4::identity()], true)
    }
}

impl<const ML: usize, LA: Pod> ContentFactory<4> for ProtoUniforms<ML, LA> {
    fn create(
        self: Box<Self>,
        [models_buffer, camera_buffer, fragment_buffer, light_buffer]: [BufferWriter; 4]
    ) -> Box<dyn Content> {
        let models: Uniform<[Model; ML], ModelUniforms> = if self.instances {
            models_buffer.to_instance_array(self.models)
        } else {
            models_buffer.to_binding_array(self.models)
        };
        Box::new(Uniforms {
            models,
            camera: camera_buffer.to_value(self.camera),
            fragment: fragment_buffer.to_value(self.fragment),
            light: light_buffer.to_value(self.light),

            animation_speed: self.animation_speed,
        })
    }
}

// Uniforms

#[allow(dead_code)]
pub struct Uniforms<const ML: usize, LA: Pod> {
    models: Uniform<[Model; ML], ModelUniforms>,
    camera: Uniform<OglCamera, CameraUniform>,
    fragment: Uniform<FragmentUniforms, FragmentUniforms>,
    light: Uniform<LightUniforms<LA>, LightUniforms<LA>>,

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
    #[must_use] pub fn new(is_two_side: bool) -> Self {
        Self {
            is_two_side: i32::from(is_two_side),
            padding: [0; 12]
        }
    }

    #[must_use] pub fn example<V: VertexBufferInfo + Into<VertexN>>(shader: &str, vertices: &[V])
        -> PipelineConfiguration
    {
        let is_two_side = CmdArgs::next_bool("Is two side", false);
        ProtoUniforms::example_aux(shader, vertices, Self::new(is_two_side))
    }
}
