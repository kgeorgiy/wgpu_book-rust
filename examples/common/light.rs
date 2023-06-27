#![allow(clippy::extra_unused_type_parameters)]

use std::f32::consts::PI;
use std::time::Duration;

use bytemuck::{Pod, Zeroable};
use cgmath::{EuclideanSpace, InnerSpace, Matrix, Matrix4, Point3, point3, Rad, SquareMatrix, Vector3};

use webgpu_book::{BufferInfo, BufferWriter, Content, ContentFactory, RenderConfiguration, UniformsConfiguration, VertexBufferInfo};
use webgpu_book::transforms::{create_projection, create_rotation};

use super::{CmdArgs, Config, Mvp, To, Uniform, VertexN};

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
    pub fn new(eye: Point3<f32>, look_at: Vector3<f32>, up: Vector3<f32>, fovy: Rad<f32>) -> Self {
        Self { eye, look_at, up, fovy, projection: create_projection(1.0, fovy) }
    }

    pub fn view(&self) -> Matrix4<f32> {
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
    pub fn new(eye: [f32; 4], light: [f32; 4]) -> Self {
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



// MvpModelView

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct MvpModelView {
    model: [[f32; 4]; 4],
    model_it: [[f32; 4]; 4],
    view_project: [[f32; 4]; 4],
}

impl To<MvpModelView> for Mvp {
    fn to(&self) -> MvpModelView {
        let model = self.model;
        MvpModelView {
            model: model.into(),
            model_it: model.invert().expect("invertible matrix").transpose().into(),
            view_project: (self.projection * self.view).into(),
        }
    }
}


// ProtoUniforms

pub struct ProtoUniforms<LA: Pod> {
    camera: OglCamera,
    fragment: FragmentUniforms,
    light: LightUniforms<LA>,
    animation_speed: f32,
    shader_source: String,
    cull_mode: Option<wgpu::Face>,
}

impl<LA: Pod> ProtoUniforms<LA> {
    pub fn new(
        camera: OglCamera,
        fragment: FragmentUniforms,
        light: LightUniforms<LA>,
        animation_speed: f32,
        shader_source: String,
        cull_mode: Option<wgpu::Face>
    ) -> Self {
        ProtoUniforms {
            camera,
            fragment,
            light,
            animation_speed,
            shader_source,
            cull_mode,
        }
    }

    pub fn example_aux(shader_source: String, cull_mode: Option<wgpu::Face>, light_aux: LA) -> Self {
        let eye = point3(3.0, 1.5, 3.0);
        let look_direction = -eye.to_vec();
        let up_direction = Vector3::unit_y();
        let fovy = Rad(2.0 * PI / 5.0);

        let side = look_direction.cross(up_direction);
        Self::new(
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

    pub fn run<V: VertexBufferInfo + Into<VertexN>>(self, title: &str, vertices: &[V]) -> ! {
        if CmdArgs::is("wireframe") {
            let vertices_n = vertices.iter().map(|v| (*v).into()).collect::<Vec<_>>();
            self.run_wireframe(title, &vertices_n, 0.1)
        } else {
            self.run_vertices(title, vertices)
        }
    }

    pub fn run_vertices<V: VertexBufferInfo>(self, title: &str, vertices: &[V]) -> ! {
        let shader_source = self.shader_source.clone();
        self.run_wgpu(
            title,
            shader_source.as_str(),
            wgpu::PrimitiveTopology::TriangleList,
            vertices,
        )
    }

    #[allow(dead_code)]
    pub fn run_wireframe(self, title: &str, vertices: &[VertexN], normal_len: f32) -> ! {
        let mut wireframe_vertices: Vec<VertexN> = Vec::with_capacity(vertices.len() * 4);
        for face in vertices.chunks(3) {
            wireframe_vertices.extend_from_slice(&[face[0], face[1], face[1], face[2], face[2], face[0]])
        }
        if normal_len > 0.0 {
            for vertex in vertices {
                wireframe_vertices.extend_from_slice(&[*vertex, vertex.normal_vertex(normal_len)]);
            }
        }

        self.run_wgpu(
            title,
            include_str!("wireframe.wgsl"),
            wgpu::PrimitiveTopology::LineList,
            &wireframe_vertices
        );
    }

    pub fn run_wgpu<V: VertexBufferInfo>(
        self,
        title: &str,
        shader_source: &str,
        topology: wgpu::PrimitiveTopology,
        vertices: &[V]
    ) -> ! {
        self.config(shader_source, topology, vertices).run_title(title);
    }

    pub fn config<V: VertexBufferInfo>(
        self,
        shader_source: &str,
        topology: wgpu::PrimitiveTopology,
        vertices: &[V]
    ) -> RenderConfiguration<3> {
        let vertex: MvpModelView = self.mvp().to();
        RenderConfiguration {
            topology,
            cull_mode: self.cull_mode,
            uniforms: UniformsConfiguration::new(
                [
                    BufferInfo::buffer_format("Vertex uniforms", &[vertex], wgpu::ShaderStages::VERTEX),
                    BufferInfo::buffer_format("Fragment uniforms", &[self.fragment], wgpu::ShaderStages::FRAGMENT),
                    BufferInfo::buffer_format("Light uniforms", &[self.light], wgpu::ShaderStages::FRAGMENT),
                ],
                Box::new(self)
            ),
            ..Config::with_vertices(shader_source, vertices, None::<&[u16]>)
        }
    }

    fn mvp(&self) -> Mvp {
        Mvp {
            model: Matrix4::identity(),
            view: self.camera.view(),
            projection: self.camera.projection(),
        }
    }
}

impl<LA: Pod> ContentFactory<3> for ProtoUniforms<LA> {
    fn create(&self, [mvp_buffer, fragment_buffer, light_buffer]: [BufferWriter; 3]) -> Box<dyn Content> {
        Box::new(Uniforms {
            camera: self.camera.clone(),
            animation_speed: self.animation_speed,
            mvp: Uniform::new(self.mvp(), mvp_buffer),
            fragment: Uniform::new(self.fragment, fragment_buffer),
            light: Uniform::new(self.light, light_buffer),
        })
    }
}

// Uniforms

#[allow(dead_code)]
pub struct Uniforms<LA: Pod> {
    camera: OglCamera,
    animation_speed: f32,

    mvp: Uniform<Mvp, MvpModelView>,
    fragment: Uniform<FragmentUniforms, FragmentUniforms>,
    light: Uniform<LightUniforms<LA>, LightUniforms<LA>>,
}

impl<LA: Pod> Content for Uniforms<LA> {
    fn resize(&mut self, width: u32, height: u32) {
        self.mvp.as_mut().projection = self.camera.resize(width, height);
    }

    fn update(&mut self, dt: Duration) {
        let angle = self.animation_speed * dt.as_secs_f32();
        self.mvp.as_mut().model = create_rotation([angle.sin(), angle.cos(), 0.0]);
        self.light.as_mut().ambient_intensity = dt.as_secs_f32() / 5.0 % 1.0;
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
    pub fn new(is_two_side: bool) -> Self {
        Self {
            is_two_side: i32::from(is_two_side),
            padding: [0; 12]
        }
    }
}
