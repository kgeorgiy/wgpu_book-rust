use std::f32::consts::PI;

use cgmath::*;

pub struct Camera {
    position: Point3<f32>,
    yaw: Rad<f32>,
    pitch: Rad<f32>,
}

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

pub struct CameraController {
    rotate_x: f32,
    rotate_y: f32,
    speed: f32,
}

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
