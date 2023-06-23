use std::f32::consts::PI;

use cgmath::Deg;
use wgpu::PrimitiveTopology;
use winit::event::{DeviceEvent, ElementState};

use webgpu_book::Content;
use webgpu_book::transforms::create_rotation;

use crate::camera::{Camera, CameraController};
use crate::state::{ColorVertex, Mvp, run_uniform};
use crate::vertex_data::FACE_COLORS_CUBE;

#[path = "../common/vertex_data.rs"]
mod vertex_data;
mod state;
mod camera;

struct CameraState {
    mvp: Mvp,
    camera: Camera,
    camera_controller: CameraController,
    mouse_pressed: bool,
}

impl Content for CameraState {
    fn resize(&mut self, width: u32, height: u32) {
        self.mvp.resize(width, height);
    }

    fn input(&mut self, event: DeviceEvent) {
        match event {
            DeviceEvent::Button {
                button: 1, // Left Mouse Button
                state,
            } => self.mouse_pressed = state == ElementState::Pressed,
            DeviceEvent::MouseMotion { delta } => {
                if self.mouse_pressed {
                    self.camera_controller.mouse_move(delta.0, delta.1);
                }
            },
            _ => (),
        }
        self.camera_controller.update_camera(&mut self.camera);
        self.mvp.set_view(self.camera.view());
    }
}

fn main() {
    let model = create_rotation([PI / 8.0, PI / 8.0, PI / 8.0]);
    run_uniform(
        "Controlled camera",
        include_str!("cube_face_colors.wgsl"),
        model,
        &ColorVertex::create(FACE_COLORS_CUBE.positions, FACE_COLORS_CUBE.colors),
        PrimitiveTopology::TriangleList,
        None,
        Box::new(move |mpv_buffer| {
            let camera = Camera::new((0.0, 0.0, -5.0), Deg(90.0), Deg(0.0));
            Box::new(CameraState {
                mvp: Mvp::new(model, camera.view(), Deg(120.0).into(), mpv_buffer),
                camera,
                camera_controller: CameraController::new(0.005),
                mouse_pressed: false,
            })
        })
    );
}
