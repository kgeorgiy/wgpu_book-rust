use std::{f32::consts::PI, time::Duration};

use bytemuck::{Pod, Zeroable};
use cgmath::{InnerSpace, Matrix4, Point3, Rad, SquareMatrix, Vector3};

use webgpu_book::{BufferInfo, BufferWriter, Content, ContentFactory, RenderConfiguration, run_wgpu_title, TypedBufferWriter, VertexBufferInfo};
use webgpu_book::transforms::{create_projection, create_rotation, create_view};

// Mvp

#[allow(dead_code)]
pub struct Mvp<S: Clone> {
    model: Matrix4<f32>,
    view: Matrix4<f32>,
    projection: Matrix4<f32>,
    fovy: Rad<f32>,
    mvp_buffer: TypedBufferWriter<[[f32; 4]; 4]>,
    pub state: S,
}

#[allow(dead_code)]
impl Mvp<()> {
    pub fn run<V: VertexBufferInfo>(
        title: &str,
        shader_source: &str,
        vertices: &[V],
        topology: wgpu::PrimitiveTopology,
        indices: Option<&[u16]>,
    ) {
        MvpProto::example(()).run(title, shader_source, vertices, topology, indices);
    }
}

impl Content for Mvp<()> {
    fn resize(&mut self, width: u32, height: u32) {
        self.resize(width, height);
    }
}

#[allow(dead_code)]
impl<S: Clone> Mvp<S> {
    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        self.projection = create_projection(width as f32 / height as f32, self.fovy);
        self.write();
    }

    pub fn set_model(&mut self, model: Matrix4<f32>) {
        self.model = model;
        self.write();
    }

    pub fn set_view(&mut self, view: Matrix4<f32>) {
        self.view = view;
        self.write();
    }

    fn write(&mut self) {
        self.mvp_buffer.write_slice(&[(self.projection * self.view * self.model).into()]);
    }
}

// MvpProto

pub struct MvpProto<S: Clone> {
    model: Matrix4<f32>,
    view: Matrix4<f32>,
    fovy: Rad<f32>,
    pub(crate) aux: S,
}

impl<S: Clone + 'static> MvpProto<S> where Mvp<S>: Content {
    pub fn new(model: Matrix4<f32>, view: Matrix4<f32>, fovy: Rad<f32>, aux: S) -> Self {
        Self { model, view, fovy, aux }
    }

    pub fn from_ogl(
        model: Matrix4<f32>,
        eye: Point3<f32>,
        look_at: Point3<f32>,
        up: Vector3<f32>,
        fovy: Rad<f32>,
        aux: S,
    ) -> Self {
        Self::new(model, create_view(eye, look_at, up), fovy, aux)
    }

    pub fn example(aux: S) -> Self {
        Self::from_ogl(
            Matrix4::identity(),
            (3.0, 1.5, 3.0).into(),
            (0.0, 0.0, 0.0).into(),
            Vector3::unit_y(),
            Rad(2.0 * PI / 5.0),
            aux,
        )
    }

    pub fn run<V: VertexBufferInfo>(
        self,
        title: &str,
        shader_source: &str,
        vertices: &[V],
        topology: wgpu::PrimitiveTopology,
        indices: Option<&[u16]>,
    ) {
        let model = self.model;
        run_wgpu_title(title, RenderConfiguration {
            shader_source: shader_source.to_string(),
            vertices: indices.map_or(vertices.len(), |indices| indices.len()),
            topology,
            vertex_buffers: vec![BufferInfo::buffer("Vertices", &vertices)],
            index_buffer: indices.map(|indices| u16::buffer("Indices", indices)),
            uniform_buffers: vec![<[[f32; 4]; 4]>::buffer("Uniform", &[model.into()])],
            content: Box::new(self),
            ..RenderConfiguration::default()
        });
    }
}

impl<S: Clone + 'static> ContentFactory for MvpProto<S> where Mvp<S>: Content{
    fn create(&self, uniforms: Vec<BufferWriter>) -> Box<dyn Content> {
        let mvp_buffer = uniforms[0].as_typed();
        let model = self.model;
        let view = self.view;
        let fovy = self.fovy;
        let aux = self.aux.clone();
        Box::new(Mvp {
            model,
            view,
            fovy,
            mvp_buffer,
            projection: create_projection(1.0, fovy),
            state: aux
        })
    }
}


// AnimationState

#[derive(Clone)]
pub struct AnimationState {
    animation_speed: f32,
}

impl AnimationState {
    #[allow(dead_code)]
    pub fn run<V: VertexBufferInfo>(
        title: &str,
        shader_source: &str,
        animation_speed: f32,
        vertices: &[V],
        topology: wgpu::PrimitiveTopology,
        indices: Option<&[u16]>,
    ) {
        MvpProto::example(Self { animation_speed })
            .run(&title, shader_source, vertices, topology, indices);
    }
}

impl Content for Mvp<AnimationState> {
    fn update(&mut self, dt: Duration) {
        let angle = self.state.animation_speed * dt.as_secs_f32();
        self.set_model(create_rotation([angle.sin(), angle.cos(), 0.0]));
    }
}


// ColorVertex

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ColorVertex {
    position: [f32; 4],
    color: [f32; 4],
}

#[allow(dead_code)]
impl ColorVertex {
    fn new(pos: [i8; 3], col: [i8; 3]) -> Self {
        ColorVertex {
            position: [pos[0] as f32, pos[1] as f32, pos[2] as f32, 1.0],
            color: [col[0] as f32, col[1] as f32, col[2] as f32, 1.0],
        }
    }

    pub fn create<const L: usize>(pos: [[i8; 3]; L], col: [[i8; 3]; L]) -> Vec<ColorVertex> {
        let mut data = Vec::with_capacity(L);
        for i in 0..pos.len() {
            data.push(ColorVertex::new(pos[i], col[i]));
        }
        data
    }
}

impl VertexBufferInfo for ColorVertex {
    const ATTRIBUTES: &'static [wgpu::VertexAttribute] = &wgpu::vertex_attr_array![0=>Float32x4, 1=>Float32x4];
}


// Camera

#[derive(Clone)]
#[allow(dead_code)]
pub struct Camera {
    position: Point3<f32>,
    yaw: Rad<f32>,
    pitch: Rad<f32>,
}

#[allow(dead_code)]
impl Camera {
    pub fn new<Pt: Into<Point3<f32>>, Yaw: Into<Rad<f32>>, Pitch: Into<Rad<f32>>>(
        position: Pt, yaw: Yaw, pitch: Pitch,
    ) -> Self {
        Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
        }
    }

    pub fn view(&self) -> Matrix4<f32> {
        let pitch = self.pitch.0;
        let yaw = self.yaw.0;
        Matrix4::look_to_rh(
            self.position,
            Vector3::new(pitch.cos() * yaw.cos(), pitch.sin(), pitch.cos() * yaw.sin()).normalize(),
            Vector3::unit_y()
        )
    }
}


// CameraController

#[derive(Clone)]
#[allow(dead_code)]
pub struct CameraController {
    rotate_x: f32,
    rotate_y: f32,
    speed: f32,
}

#[allow(dead_code)]
impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            rotate_x: 0.0,
            rotate_y: 0.0,
            speed,
        }
    }

    pub fn mouse_move(&mut self, mouse_x: f64, mouse_y: f64) {
        self.rotate_x = mouse_x as f32;
        self.rotate_y = mouse_y as f32;
    }

    pub fn update_camera(&mut self, camera: &mut Camera) {
        camera.yaw += Rad(self.rotate_x) * self.speed;
        camera.pitch += Rad(self.rotate_y) * self.speed;
        self.rotate_x = 0.0;
        self.rotate_y = 0.0;
        if camera.pitch < -Rad(PI / 3.0) {
            camera.pitch = -Rad(PI / 3.0);
        } else if camera.pitch > Rad(PI / 3.0) {
            camera.pitch = Rad(PI / 3.0);
        }
    }
}
