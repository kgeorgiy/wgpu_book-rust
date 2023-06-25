use std::f32::consts::PI;

use cgmath::Deg;
use wgpu::PrimitiveTopology;
use winit::event::{DeviceEvent, ElementState};

use webgpu_book::Content;
use webgpu_book::transforms::create_rotation;

use crate::common06::{Camera, CameraController, ColorVertex, Mvp, MvpProto};
use crate::vertex_data::FACE_COLORS_CUBE;

#[path = "../common/vertex_data.rs"]
mod vertex_data;
mod common06;

#[derive(Clone)]
struct CameraState {
    camera: Camera,
    camera_controller: CameraController,
    mouse_pressed: bool,
}

impl CameraState {
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
    }
}

impl Content for Mvp<CameraState> {
    fn input(&mut self, event: DeviceEvent) {
        self.state.input(event);
        self.set_view(self.state.camera.view());
    }
}

fn main() {
    let model = create_rotation([PI / 8.0, PI / 8.0, PI / 8.0]);
    let camera = Camera::new((0.0, 0.0, -5.0), Deg(90.0), Deg(0.0));
    let proto = MvpProto::new(model, camera.view(), Deg(120.0).into(), CameraState {
        camera,
        camera_controller: CameraController::new(0.005),
        mouse_pressed: false,
    });
    proto.run(
        "Ch6. Controlled camera",
        include_str!("cube_face_colors.wgsl"),
        &ColorVertex::create(FACE_COLORS_CUBE.positions, FACE_COLORS_CUBE.colors),
        PrimitiveTopology::TriangleList,
        None
    );
    // run_uniform(
    //     "Ch6. Controlled camera",
    //     include_str!("cube_face_colors.wgsl"),
    //     model,
    //     &ColorVertex::create(FACE_COLORS_CUBE.positions, FACE_COLORS_CUBE.colors),
    //     PrimitiveTopology::TriangleList,
    //     None,
    //     Box::new(move |mpv_buffer| {
    //         Box::new(CameraState {
    //             mvp: Mvp::new(model, camera.view(), Deg(120.0).into(), mpv_buffer),
    //             camera,
    //             camera_controller: CameraController::new(0.005),
    //             mouse_pressed: false,
    //         })
    //     })
    // );
}
