use core::time::Duration;
use bytemuck::{Pod, Zeroable};

use cgmath::{Deg, Matrix4, Point3, Rad, SquareMatrix, Vector3};

use webgpu_book::{Configurator, Content, func_box, PipelineConfiguration, To, typed_box, Uniform, UniformInfo};
use webgpu_book::boxed::FuncBox;
use webgpu_book::transforms::{create_projection, create_rotation, create_view};


// Mvp

struct Mvp {
    model: Matrix4<f32>,
    view: Matrix4<f32>,
    projection: Matrix4<f32>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct MvpMatrix {
    matrix: [[f32; 4]; 4],
}

impl UniformInfo for MvpMatrix {
    const STRUCT_NAME: &'static str = "MvpMatrix";
    const BINDING_NAME: &'static str = "mvp_u";
    const ATTRIBUTES: &'static [(&'static str, &'static str)] = &[
        ("matrix", "mat4x4<f32>"),
    ];
}

impl To<MvpMatrix> for Mvp {
    fn to(&self) -> MvpMatrix {
        MvpMatrix{ matrix: (self.projection * self.view * self.model).into() }
    }
}



// MvpController

pub struct MvpController<T> {
    mvp: Uniform<Mvp>,
    fovy: Rad<f32>,
    pub(crate) state: T,
}

impl Content<()> for MvpController<()> {
    fn resize(&mut self, _content: (), width: u32, height: u32) {
        self.mvp.as_mut().projection = create_projection(width as f32 / height as f32, self.fovy);
    }
}

#[allow(dead_code)]
impl<T> MvpController<T> {
    pub fn set_model(&mut self, model: Matrix4<f32>) {
        self.mvp.as_mut().model = model;
    }

    pub fn set_view(&mut self, view: Matrix4<f32>) {
        self.mvp.as_mut().view = view;
    }
}

impl<T: 'static> MvpController<T> where MvpController<T>: Content<()> {
    pub fn from_model_view(model: Matrix4<f32>, view: Matrix4<f32>, fovy: Rad<f32>, state: T)
        -> Configurator<PipelineConfiguration>
    {
        func_box!(move |mut pipeline: PipelineConfiguration| {
            let mvp_s = Mvp { model, view, projection: create_projection(1.0, fovy) };
            let mvp: Uniform<Mvp> = pipeline.uniforms().add("Mvp", mvp_s, wgpu::ShaderStages::VERTEX).value();
            pipeline.add_listener(typed_box!(dyn Content<()>, MvpController { mvp, fovy, state }));
            pipeline
        })
    }

    pub fn from_ogl<P: Into<Point3<f32>>, F: Into<Rad<f32>>>(
        model: Matrix4<f32>,
        eye: P,
        look_at: P,
        up: Vector3<f32>,
        fovy: F,
        value: T,
    ) -> Configurator<PipelineConfiguration> {
        Self::from_model_view(model, create_view(eye.into(), look_at.into(), up), fovy.into(), value)
    }

    pub fn example(value: T) -> Configurator<PipelineConfiguration> {
        Self::from_ogl(
            Matrix4::identity(),
            (5.0, 1.5, 3.0),
            (0.0, 0.0, 0.0),
            Vector3::unit_y(),
            Deg(45.0),
            value,
        )
    }
}


// AnimationState

#[derive(Clone)]
pub struct AnimationState {
    animation_speed: f32,
}

impl AnimationState {
    pub fn example() -> Configurator<PipelineConfiguration> {
        MvpController::example(AnimationState { animation_speed: 1.0 })
    }
}

impl Content<()> for MvpController<AnimationState> {
    fn update(&mut self, _context: (), dt: Duration) {
        let (sin, cos) = (self.state.animation_speed * dt.as_secs_f32()).sin_cos();
        self.set_model(create_rotation([Rad(sin), Rad(cos), Rad(0.0)]));
    }
}
