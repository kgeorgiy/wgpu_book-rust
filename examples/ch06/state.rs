use std::f32::consts::PI;
use std::time::Duration;
use cgmath::{Matrix4, Point3, Rad, SquareMatrix, Vector3};
use wgpu::{PrimitiveTopology, VertexAttribute};
use bytemuck::{Pod, Zeroable};

use webgpu_book::{
    BufferInfo, BufferWriter, Content, RenderConfiguration, run_wgpu, TypedBufferDescriptor,
    VertexBufferInfo, WindowConfiguration,
};
use webgpu_book::transforms::{create_projection, create_rotation, create_view};

// Mvp

#[allow(dead_code)]
pub struct Mvp {
    model: Matrix4<f32>,
    view: Matrix4<f32>,
    projection: Matrix4<f32>,
    fovy: Rad<f32>,
    mvp_buffer: BufferWriter<[[f32; 4]; 4]>,
}

#[allow(dead_code)]
impl Mvp {
    pub(crate) fn new(model: Matrix4<f32>, view: Matrix4<f32>, fovy: Rad<f32>, mvp_buffer: BufferWriter<[[f32; 4]; 4]>) -> Self {
        Mvp {
            model,
            view,
            fovy,
            mvp_buffer,
            projection: create_projection(1.0, fovy),
        }
    }

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

pub(crate) struct MvpProto {
    model: Matrix4<f32>,
    view: Matrix4<f32>,
    fovy: Rad<f32>,
}

impl MvpProto {
    pub(crate) fn create(&self, mvp_buffer: BufferWriter<[[f32; 4]; 4]>) -> Mvp {
        Mvp::new(self.model, self.view, self.fovy, mvp_buffer)
    }

    pub fn new(model: Matrix4<f32>, view: Matrix4<f32>, fovy: Rad<f32>) -> Self {
        MvpProto { model, view, fovy }
    }

    pub fn from_ogl(
        model: Matrix4<f32>,
        eye: Point3<f32>,
        look_at: Point3<f32>,
        up: Vector3<f32>,
        fovy: Rad<f32>,
    ) -> Self {
        Self::new(model, create_view(eye, look_at, up), fovy)
    }

    pub fn example() -> Self {
        Self::from_ogl(
            Matrix4::identity(),
            (3.0, 1.5, 3.0).into(),
            (0.0, 0.0, 0.0).into(),
            Vector3::unit_y(),
            Rad(2.0 * PI / 5.0),
        )
    }

    pub(crate) fn run<V: VertexBufferInfo>(
        title: &str,
        shader_source: &str,
        mvp_proto: MvpProto,
        vertices: &[V],
        topology: PrimitiveTopology,
        indices: Option<&[u16]>,
        content: Box<dyn FnOnce(Mvp) -> Box<dyn Content + 'static>>
    ) {
        run_uniform(
            title, shader_source, mvp_proto.model, vertices, topology, indices,
            Box::new(move |buffer| content(mvp_proto.create(buffer))),
        );
    }
}


// MvpState

#[allow(dead_code)]
pub struct MvpState {
    mvp: Mvp,
}

#[allow(dead_code)]
impl MvpState {
    pub fn run<V: VertexBufferInfo>(
        title: &str,
        shader_source: &str,
        vertices: &[V],
        topology: PrimitiveTopology,
        indices: Option<&[u16]>,
    ) {
        MvpProto::run(
            title, shader_source, MvpProto::example(), vertices, topology, indices,
            Box::new(move |mvp| Box::new(MvpState { mvp })),
        );
    }
}

impl Content for MvpState {
    fn resize(&mut self, width: u32, height: u32) {
        self.mvp.resize(width, height);
    }
}


// Animation state

pub struct AnimationState {
    mvp: Mvp,
    animation_speed: f32,
}

impl AnimationState {
    #[allow(dead_code)]
    pub fn run<V: VertexBufferInfo>(
        title: &str,
        shader_source: &str,
        animation_speed: f32,
        vertices: &[V],
        topology: PrimitiveTopology,
        indices: Option<&[u16]>,
    ) {
        MvpProto::run(
            &title, shader_source, MvpProto::example(), vertices, topology, indices,
            Box::new(move |mvp| Box::new(AnimationState { mvp, animation_speed }))
        );
    }
}

impl Content for AnimationState {
    fn resize(&mut self, width: u32, height: u32) {
        self.mvp.resize(width, height);
    }

    fn update(&mut self, dt: Duration) {
        let angle = self.animation_speed * dt.as_secs_f32();
        self.mvp.set_model(create_rotation([angle.sin(), angle.cos(), 0.0]));
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
    const ATTRIBUTES: &'static [VertexAttribute] = &wgpu::vertex_attr_array![0=>Float32x4, 1=>Float32x4];
}

pub fn run_uniform<V: VertexBufferInfo>(
    title: &str,
    shader_source: &str,
    model: Matrix4<f32>,
    vertices: &[V],
    topology: PrimitiveTopology,
    indices: Option<&[u16]>,
    content: Box<dyn FnOnce(BufferWriter<[[f32; 4]; 4]>) -> Box<dyn Content + 'static>>
) {
    run_wgpu(
        &WindowConfiguration { title: format!("Ch6. {}", title).as_str() },
        RenderConfiguration {
            shader_source,
            vertices: indices.map_or(vertices.len(), |indices| indices.len()),
            vertex_buffers: &[V::buffer("Vertices", &vertices)],
            index_buffer: indices.map(|indices| u16::buffer("Indices", indices)),
            topology,
            uniform_buffer: Some(TypedBufferDescriptor::new(
                "Uniform",
                &[model.into()],
                wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                (),
            )),
            content,
            ..RenderConfiguration::default()
        },
    );
}
