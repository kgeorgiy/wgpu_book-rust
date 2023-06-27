// Mvp

use std::marker::PhantomData;
use std::time::Duration;

use bytemuck::{Pod, Zeroable};
use cgmath::{Deg, Matrix4, Point3, point3, Rad, SquareMatrix, Vector3};

use webgpu_book::{BufferInfo, BufferWriter, Content, ContentFactory, IndexBufferInfo, RenderConfiguration, UniformsConfiguration, VertexBufferInfo};
use webgpu_book::transforms::{create_projection, create_rotation, create_view};

use super::{Config, To, Uniform};

#[derive(Clone)]
pub struct Mvp {
    pub(crate) model: Matrix4<f32>,
    pub(crate) view: Matrix4<f32>,
    pub(crate) projection: Matrix4<f32>,
}


#[allow(dead_code)]
pub struct MvpContent<T, B: Pod> where Mvp: To<B> {
    pub mvp: Uniform<Mvp, B>,
    pub fovy: Rad<f32>,
    pub state: T,
}

impl<B: Pod> Content for MvpContent<(), B> where Mvp: To<B> {
    fn resize(&mut self, width: u32, height: u32) {
        MvpContent::resize(self, width, height);
    }
}

#[allow(dead_code)]
impl<'a, B: Pod, T> MvpContent<T, B> where Mvp: To<B> {
    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        self.mvp.as_mut().projection = create_projection(width as f32 / height as f32, self.fovy);
    }

    pub fn set_model(&mut self, model: Matrix4<f32>) {
        self.mvp.as_mut().model = model;
    }

    pub fn set_view(&mut self, view: Matrix4<f32>) {
        self.mvp.as_mut().view = view;
    }
}


// MvpFactory

pub struct MvpFactory<T, B> {
    mvp: Mvp,
    fovy: Rad<f32>,
    state: T,
    phantom: PhantomData<B>,
}

impl<B, T: Clone + 'static> MvpFactory<T, B> {
    pub fn new(model: Matrix4<f32>, view: Matrix4<f32>, fovy: Rad<f32>, state: T) -> Self {
        let mvp = Mvp { model, view, projection: create_projection(1.0, fovy) };
        Self { mvp, fovy, state, phantom: Default::default() }
    }

    pub fn from_ogl<P: Into<Point3<f32>>, F: Into<Rad<f32>>>(
        model: Matrix4<f32>,
        eye: P,
        look_at: P,
        up: Vector3<f32>,
        fovy: F,
        value: T,
    ) -> Self {
        Self::new(model, create_view(eye.into(), look_at.into(), up), fovy.into(), value)
    }

    pub fn example(value: T) -> Self {
        Self::from_ogl(
            Matrix4::identity(),
            point3(3.0, 1.5, 3.0),
            point3(0.0, 0.0, 0.0),
            Vector3::unit_y(),
            Deg(45.0),
            value,
        )
    }
}

impl<B: Pod + 'static, T: Clone + 'static> MvpFactory<T, B> where Mvp: To<B>, MvpContent<T, B>: Content {
    pub fn run<V: VertexBufferInfo, I: IndexBufferInfo>(
        self,
        title: &str,
        shader_source: &str,
        vertices: &[V],
        topology: wgpu::PrimitiveTopology,
        indices: Option<&[I]>,
    ) -> ! {
        RenderConfiguration {
            topology,
            uniforms: UniformsConfiguration::new(
                [<B>::buffer("Uniform", &[self.mvp.to()])],
                Box::new(self)
            ),
            ..Config::with_vertices(shader_source, vertices, indices)
        }.run_title(title);
    }
}

impl<B: Pod + 'static, S: Clone + 'static> ContentFactory<1> for MvpFactory<S, B>
    where MvpContent<S, B>: Content, Mvp: To<B>
{
    fn create(&self, [mvp_buffer]: [BufferWriter; 1]) -> Box<dyn Content> {
        Box::new(MvpContent {
            mvp: Uniform::new(self.mvp.clone(), mvp_buffer),
            fovy: self.fovy,
            state: self.state.clone()
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
    pub fn run<B: Pod, V: VertexBufferInfo, I: IndexBufferInfo>(
        title: &str,
        shader_source: &str,
        animation_speed: f32,
        vertices: &[V],
        topology: wgpu::PrimitiveTopology,
        indices: Option<&[I]>,
    ) -> ! where Mvp: To<B> {
        MvpFactory::example(AnimationState { animation_speed })
            .run(&title, shader_source, vertices, topology, indices);
    }
}

impl<B: Pod> Content for MvpContent<AnimationState, B> where Mvp: To<B> {
    fn update(&mut self, dt: Duration) {
        let (sin, cos) = (self.state.animation_speed * dt.as_secs_f32()).sin_cos();
        self.set_model(create_rotation([sin, cos, 0.0]));
    }
}

// MvpMvp

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct MvpMatrix {
    mvp: [[f32; 4]; 4],
}

impl To<MvpMatrix> for Mvp {
    fn to(&self) -> MvpMatrix {
        MvpMatrix { mvp: (self.projection * self.view * self.model).into() }
    }
}
