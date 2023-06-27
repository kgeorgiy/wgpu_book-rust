#![allow(clippy::extra_unused_type_parameters, clippy::module_name_repetitions)]

use core::f32::consts::PI;
use core::time::Duration;

use bytemuck::{Pod, Zeroable};
use cgmath::{Angle, EuclideanSpace, InnerSpace, Matrix, Matrix4, Point3, point3, Rad, SquareMatrix, Vector3};

use webgpu_book::{BufferInfo, BufferWriter, Content, ContentFactory, PipelineConfiguration, VertexBufferInfo};
use webgpu_book::transforms::{create_projection, create_rotation};

use super::{CmdArgs, To, Uniform, VertexN};


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

impl<const L: usize> To<[ModelUniforms; L]> for [Model; L] {
    fn to(&self) -> [ModelUniforms; L] {
        self.clone().map(|model| model.to())
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
    camera: OglCamera,
    fragment: FragmentUniforms,
    light: LightUniforms<LA>,
    animation_speed: f32,
    shader_source: String,
    cull_mode: Option<wgpu::Face>,
}

impl<const ML: usize, LA: Pod> ProtoUniforms<ML, LA> {
    pub fn new(
        models: [Model; ML],
        camera: OglCamera,
        fragment: FragmentUniforms,
        light: LightUniforms<LA>,
        animation_speed: f32,
        shader_source: String,
        cull_mode: Option<wgpu::Face>
    ) -> Self {
        ProtoUniforms {
            models,
            camera,
            fragment,
            light,
            animation_speed,
            shader_source,
            cull_mode,
        }
    }

    pub fn run<V: VertexBufferInfo + Into<VertexN>>(self, title: &str, vertices: &[V]) -> ! {
        self.into_advanced_config(vertices).run_title(title)
    }

    fn into_advanced_config<V: VertexBufferInfo + Into<VertexN>>(self, vertices: &[V]) -> PipelineConfiguration {
        if CmdArgs::is("wireframe") {
            let vertices_n = vertices.iter().map(|v| (*v).into()).collect::<Vec<_>>();
            self.wireframe(&vertices_n, 0.1)
        } else {
            println!("Here");
            self.into_config().with_vertices(vertices)
        }
    }

    fn wireframe(self, vertices: &[VertexN], normal_len: f32) -> PipelineConfiguration {
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

        self.into_config()
            .with_shader(include_str!("wireframe.wgsl"))
            .with_vertices(&wireframe_vertices)
            .with_topology(wgpu::PrimitiveTopology::LineList)
    }

    pub fn into_config(self) -> PipelineConfiguration {
        let models: [ModelUniforms; ML] = self.models.to();
        let camera: CameraUniform = self.camera.to();
        PipelineConfiguration::new(self.shader_source.as_str())
            .with_cull_mode(self.cull_mode)
            .with_instances(ML)
            .with_uniforms(
                [
                    BufferInfo::buffer_format("Models uniform", &[models], wgpu::ShaderStages::VERTEX),
                    BufferInfo::buffer_format("Camera uniform", &[camera], wgpu::ShaderStages::VERTEX),
                    BufferInfo::buffer_format("Fragment uniform", &[self.fragment], wgpu::ShaderStages::FRAGMENT),
                    BufferInfo::buffer_format("Light uniform", &[self.light], wgpu::ShaderStages::FRAGMENT),
                ],
                Box::new(self),
            )
    }

    pub fn example_models(shader_source: String, cull_mode: Option<wgpu::Face>, light_aux: LA, models: [Matrix4<f32>; ML]) -> ProtoUniforms<ML, LA> {
        let eye = point3(3.0, 1.5, 3.0);
        let look_direction = -eye.to_vec();
        let up_direction = Vector3::unit_y();
        let fovy = Rad(2.0 * PI / 5.0);

        let side = look_direction.cross(up_direction);
        Self::new(
            models.map(Model::new),
            OglCamera::new(eye, look_direction, up_direction, fovy),
            FragmentUniforms::new(
                eye.to_homogeneous().into(),
                (side.normalize() - look_direction.normalize() * 2.0).extend(0.0).into()
            ),
            LightUniforms::new(point3(1.0, 1.0, 0.0), 0.1, 1.0, 2.0, 30.0, light_aux),
            1.0,
            shader_source,
            cull_mode,
        )
    }
}

impl<LA: Pod> ProtoUniforms<1, LA> {
    pub fn example_aux(shader_source: String, cull_mode: Option<wgpu::Face>, light_aux: LA) -> Self {
        Self::example_models(shader_source, cull_mode, light_aux, [Matrix4::identity()])
    }
}

impl<const ML: usize, LA: Pod> ContentFactory<4> for ProtoUniforms<ML, LA> {
    fn create(
        self: Box<Self>,
        [models_buffer, view_buffer, fragment_buffer, light_buffer]: [BufferWriter; 4]
    ) -> Box<dyn Content> {
        Box::new(Uniforms {
            models: Uniform::new(self.models, models_buffer),
            camera: Uniform::new(self.camera, view_buffer),
            fragment: Uniform::new(self.fragment, fragment_buffer),
            light: Uniform::new(self.light, light_buffer),

            animation_speed: self.animation_speed,
        })
    }
}

// Uniforms

#[allow(dead_code)]
pub struct Uniforms<const ML: usize, LA: Pod> {
    models: Uniform<[Model; ML], [ModelUniforms; ML]>,
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

    #[must_use] pub fn example(shader: &str) -> ProtoUniforms<1, TwoSideLightAux> {
        let is_two_side = CmdArgs::next("false").parse().expect("true of false");
        ProtoUniforms::example_aux(
            shader.to_owned(),
            None,
            Self::new(is_two_side),
        )
    }
}
