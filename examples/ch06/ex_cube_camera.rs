use core::f32::consts::PI;

use bytemuck::Pod;
use cgmath::Deg;
use winit::event::{DeviceEvent, ElementState};

use webgpu_book::{Content, transforms::create_rotation};

use crate::common::{Camera, CameraController, create_vertices, Mvp, MvpContent, MvpFactory, MvpMatrix, To, VertexC};
use crate::common::vertex_data::FACE_COLORS_CUBE;

mod common;

#[derive(Clone)]
struct CameraState {
    camera: Camera,
    camera_controller: CameraController,
    mouse_pressed: bool,
}

impl CameraState {
    fn input(&mut self, event: &DeviceEvent) {
        match *event {
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
    }
}

impl<B: Pod> Content for MvpContent<CameraState, B> where Mvp: To<B> {
    fn input(&mut self, event: &DeviceEvent) {
        self.state.input(event);
        self.set_view(self.state.camera.view());
    }
}

fn main() {
    let model = create_rotation([PI / 8.0, PI / 8.0, PI / 8.0]);
    let camera = Camera::new((0.0, 0.0, -5.0), Deg(90.0), Deg(0.0));
    MvpFactory::<CameraState, MvpMatrix>::new(model, camera.view(), Deg(120.0).into(), CameraState {
        camera,
        camera_controller: CameraController::new(0.005),
        mouse_pressed: false,
    }).run::<VertexC, u16>(
        "Chapter 6 Controlled camera",
        include_str!("cube_face_colors.wgsl"),
        &create_vertices(FACE_COLORS_CUBE.positions, FACE_COLORS_CUBE.colors),
        wgpu::PrimitiveTopology::TriangleList,
        None
    );
}
