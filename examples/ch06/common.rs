use core::f32::consts::PI;
use core::iter::zip;

use cgmath::{Angle, InnerSpace, Matrix4, Point3, Rad, Vector3};

use webgpu_book::{PipelineConfiguration, VertexBufferInfo};
use crate::common::mvp::MvpController;
use crate::common::surface_data::Quads;

use crate::common::vertex_data::i8_as_f32;

pub use self::global_common::*;

#[path = "../common/global_common.rs"]
mod global_common;

#[allow(dead_code)]
pub fn run_example<V: VertexBufferInfo>(
    title: &str,
    shader_source: &str,
    vertices: Vec<V>,
    topology: wgpu::PrimitiveTopology,
    indices: Option<&[u16]>,
) {
    PipelineConfiguration::new(shader_source)
        .with(MvpController::example(()))
        .with_vertices_indices(vertices, indices)
        .with_topology(topology)
        .run_title(title);
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
        let (pitch_sin, pitch_cos) = self.pitch.sin_cos();
        let (yaw_sin, yaw_cos) = self.yaw.sin_cos();
        Matrix4::look_to_rh(
            self.position,
            Vector3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize(),
            Vector3::unit_y(),
        )
    }
}


// CameraController

#[derive(Clone)]
#[allow(dead_code)]
pub struct CameraController {
    rotate_x: f32,
    rotate_y: f32,
    speed: Rad<f32>,
}

#[allow(dead_code)]
impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            rotate_x: 0.0,
            rotate_y: 0.0,
            speed: Rad(speed),
        }
    }

    pub fn mouse_move(&mut self, mouse_x: f64, mouse_y: f64) {
        #![allow(clippy::cast_possible_truncation)]
        self.rotate_x = mouse_x as f32;
        self.rotate_y = mouse_y as f32;
    }

    pub fn update_camera(&mut self, camera: &mut Camera) {
        camera.yaw += self.speed * self.rotate_x;
        camera.pitch += self.speed * self.rotate_y;
        camera.pitch = Rad(camera.pitch.0.clamp(-PI / 3.0, PI / 3.0));
        self.rotate_x = 0.0;
        self.rotate_y = 0.0;
    }
}

//
// Other

#[allow(dead_code, clippy::indexing_slicing)]
pub fn create_cube<const L: usize>(positions: [[[i8; 3]; 4]; L], colors: [[[i8; 3]; 4]; L]) -> Quads<VertexC> {
    zip(i8_as_f32(positions), i8_as_f32(colors))
        .map(|(pos, cols)|
            [0, 1, 2, 3].map(|i| VertexC::new(pos[i], cols[i])))
        .into()
}
