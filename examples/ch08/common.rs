use std::f32::consts::PI;
use std::time::Duration;

use bytemuck::{Pod, Zeroable};
use cgmath::{EuclideanSpace, InnerSpace, Matrix, Matrix4, Point3, point3, Rad, SquareMatrix, Vector3};
use wgpu::{PrimitiveTopology, ShaderStages, VertexAttribute};

use webgpu_book::{BufferInfo, Content, RenderConfiguration, run_wgpu, TypedBufferWriter, VertexBufferInfo, WindowConfiguration};
use webgpu_book::transforms::{create_projection, create_rotation};

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

    pub fn projection(&self) -> Matrix4<f32> {
        self.projection
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.projection = create_projection(width as f32 / height as f32, self.fovy);
    }
}


// VertexUniforms

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct VertexUniforms {
    model: [[f32; 4]; 4],
    model_it: [[f32; 4]; 4],
    view_project: [[f32; 4]; 4],
}

impl VertexUniforms {
    pub fn new(model: Matrix4<f32>, view_project: Matrix4<f32>) -> Self {
        Self {
            model: model.into(),
            model_it: model.invert().unwrap().transpose().into(),
            view_project: view_project.into(),
        }
    }

    pub(crate) fn set_view_project(&mut self, writer: &TypedBufferWriter<VertexUniforms>, view_project: Matrix4<f32>) {
        self.view_project = view_project.into();
        writer.write_slice(&[*self]);
    }

    pub(crate) fn set_model(&mut self, writer: &TypedBufferWriter<VertexUniforms>, model: Matrix4<f32>) {
        self.model = model.into();
        self.model_it = model.invert().unwrap().transpose().into();
        writer.write_slice(&[*self]);
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

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct LightUniforms {
    color: [f32; 4],
    specular_color: [f32; 4],
    ambient_intensity: f32,
    diffuse_intensity: f32,
    specular_intensity: f32,
    specular_shininess: f32,
}

impl LightUniforms {
    pub fn new(
        color: Point3<f32>,
        specular_color: Point3<f32>,
        ambient_intensity: f32,
        diffuse_intensity: f32,
        specular_intensity: f32,
        specular_shininess: f32
    ) -> Self {
        Self {
            color: color.to_homogeneous().into(),
            specular_color: specular_color.to_homogeneous().into(),
            ambient_intensity,
            diffuse_intensity,
            specular_intensity,
            specular_shininess,
        }
    }
}


// ProtoUniforms

pub struct ProtoUniforms {
    camera: OglCamera,
    vertex: VertexUniforms,
    fragment: FragmentUniforms,
    light: LightUniforms,
    animation_speed: f32,
}

impl ProtoUniforms {
    pub fn new(
        camera: OglCamera,
        fragment: FragmentUniforms,
        light: LightUniforms,
        animation_speed: f32
    ) -> Self {
        let view_project = camera.projection() * camera.view();
        ProtoUniforms {
            camera,
            vertex: VertexUniforms::new(Matrix4::identity(), view_project),
            fragment,
            light,
            animation_speed,
        }
    }

    pub fn example() -> Self {
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
            LightUniforms::new(point3(1.0, 0.0, 0.0), point3(1.0, 1.0, 0.0), 0.1, 1.0, 2.0, 30.0),
            1.0,
        )
    }

    pub fn run<V: VertexBufferInfo>(self, title: &str, vertices: &[V]) {
        run_wgpu(
            &WindowConfiguration { title },
            RenderConfiguration {
                shader_source: include_str!("shader.wgsl"),
                vertices: vertices.len(),
                topology: PrimitiveTopology::TriangleList,
                vertex_buffers: &[V::buffer("Vertices", &vertices)],
                index_buffer: None,
                uniform_buffers: &[
                    BufferInfo::buffer_format("Vertex uniforms", &[self.vertex], ShaderStages::VERTEX),
                    BufferInfo::buffer_format("Fragment uniforms", &[self.fragment], ShaderStages::FRAGMENT),
                    BufferInfo::buffer_format("Light uniforms", &[self.light], ShaderStages::FRAGMENT),
                ],
                content: Box::new(move |buffers| Box::new(Uniforms {
                    camera: self.camera.clone(),
                    animation_speed: self.animation_speed,

                    vertex: self.vertex,
                    vertex_writer: buffers[0].as_typed(),
                    fragment: self.fragment,
                    fragment_writer: buffers[1].as_typed(),
                    light: self.light,
                    light_writer: buffers[2].as_typed(),
                })),
                ..RenderConfiguration::default()
            },
        );
    }
}

// Uniforms

#[allow(dead_code)]
pub struct Uniforms {
    camera: OglCamera,
    animation_speed: f32,

    vertex: VertexUniforms,
    fragment: FragmentUniforms,
    light: LightUniforms,
    vertex_writer: TypedBufferWriter<VertexUniforms>,
    fragment_writer: TypedBufferWriter<FragmentUniforms>,
    light_writer: TypedBufferWriter<LightUniforms>,
}

impl Content for Uniforms {
    fn resize(&mut self, width: u32, height: u32) {
        self.camera.resize(width, height);
        self.vertex.set_view_project(&self.vertex_writer, self.camera.projection() * self.camera.view())
    }

    fn update(&mut self, dt: Duration) {
        let angle = self.animation_speed * dt.as_secs_f32();
        self.vertex.set_model(&self.vertex_writer, create_rotation([angle.sin(), angle.cos(), 0.0]));
    }
}


// Vertex

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 4],
    pub normal: [f32; 4],
}

impl Vertex {
    #[allow(dead_code)]
    pub fn new(position: Point3<f32>, normal: Vector3<f32>) -> Self {
        Self {
            position: position.to_homogeneous().into(),
            normal: normal.normalize().extend(0.0).into(),
        }
    }
}


impl VertexBufferInfo for Vertex {
    const ATTRIBUTES: &'static [VertexAttribute] = &wgpu::vertex_attr_array![0=>Float32x4, 1=>Float32x4];
}
