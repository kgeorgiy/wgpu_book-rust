use std::f32::consts::PI;
use std::iter::zip;

use cgmath::{InnerSpace, Matrix4, Point3, Rad, Vector3};

use webgpu_book::VertexBufferInfo;
use crate::common::vertex_data::i8_as_f32;

pub use self::global_common::*;

#[path = "../common/global_common.rs"]
mod global_common;

#[allow(dead_code)]
pub fn run_example<V: VertexBufferInfo>(
    title: &str,
    shader_source: &str,
    vertices: &[V],
    topology: wgpu::PrimitiveTopology,
    indices: Option<&[u16]>,
) {
    MvpFactory::<(), MvpMatrix>::example(()).run(title, shader_source, vertices, topology, indices);
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

// Other

#[allow(dead_code)]
pub fn create_vertices<const L: usize>(positions: [[i8; 3]; L], colors: [[i8; 3]; L]) -> Vec<VertexC> {
    zip(i8_as_f32(positions), i8_as_f32(colors))
        .map(|(pos, col)| VertexC::new(pos, col))
        .collect()
}
