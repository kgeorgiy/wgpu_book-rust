use cgmath::{Angle, Deg, Rad};
use winit::event::{DeviceEvent, ElementState};

use webgpu_book::{Content, PipelineConfiguration, transforms::create_rotation};

use crate::common::{Camera, CameraController, create_cube};
use crate::common::mvp::MvpController;
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
            }
            _ => (),
        }
        self.camera_controller.update_camera(&mut self.camera);
    }
}

impl Content<()> for MvpController<CameraState> {
    fn input(&mut self, _context: (), event: &DeviceEvent) {
        self.state.input(event);
        self.set_view(self.state.camera.view());
    }
}

fn main() {
    let angle = Rad::full_turn() / 16.0;
    let model = create_rotation([angle, angle, angle]);
    let camera = Camera::new((0.0, 0.0, -5.0), Deg(90.0), Deg(0.0));
    let cube = FACE_COLORS_CUBE;
    PipelineConfiguration::new(include_str!("cube_face_colors.wgsl"))
        .with(MvpController::from_model_view(model, camera.view(), Deg(90.0).into(), CameraState {
            camera,
            camera_controller: CameraController::new(0.005),
            mouse_pressed: false,
        }))
        .with(create_cube(cube.positions, cube.colors).triangles().vertices())
        .run_title("Chapter 6 Controlled camera");
}
