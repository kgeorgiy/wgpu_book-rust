use cgmath::{Matrix4, Point3, Rad, Vector3};

pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 1.0,
);

pub fn create_projection(aspect: f32, fovy: Rad<f32>) -> Matrix4<f32> {
    let project = if fovy.0 > 0.0 {
        cgmath::perspective(fovy, aspect, 0.1, 100.0)
    } else {
        cgmath::ortho(-4.0, 4.0, -3.0, 3.0, -1.0, 6.0)
    };

    OPENGL_TO_WGPU_MATRIX * project
}

pub fn create_view(eye: Point3<f32>, look_at: Point3<f32>, up: Vector3<f32>) -> Matrix4<f32> {
    Matrix4::look_at_rh(eye, look_at, up)
}

pub fn create_rotation(rotation: [f32; 3]) -> Matrix4<f32> {
    let rotate_x = Matrix4::from_angle_x(Rad(rotation[0]));
    let rotate_y = Matrix4::from_angle_y(Rad(rotation[1]));
    let rotate_z = Matrix4::from_angle_z(Rad(rotation[2]));
    rotate_z * rotate_y * rotate_x
}

pub fn create_transforms(
    translation: [f32; 3],
    rotation: [f32; 3],
    scaling: [f32; 3],
) -> Matrix4<f32> {
    let trans = Matrix4::from_translation(translation.into());
    let rot = create_rotation(rotation);
    let scale = Matrix4::from_nonuniform_scale(scaling[0], scaling[1], scaling[2]);
    trans * rot * scale
}
