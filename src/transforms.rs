use cgmath::{Matrix4, Point3, Rad, Vector3, Zero};

pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 1.0,
);

#[must_use] pub fn create_projection(aspect: f32, fovy: Rad<f32>) -> Matrix4<f32> {
    let project = if fovy > Rad::zero() {
        cgmath::perspective(fovy, aspect, 0.1, 100.0)
    } else {
        let view = 1.5;
        cgmath::ortho(-view * aspect, view * aspect, -view, view, 0.0, 100.0)
    };

    OPENGL_TO_WGPU_MATRIX * project
}

#[must_use] pub fn create_view(eye: Point3<f32>, look_at: Point3<f32>, up: Vector3<f32>) -> Matrix4<f32> {
    Matrix4::look_at_rh(eye, look_at, up)
}

#[must_use] pub fn create_rotation(rotation: [Rad<f32>; 3]) -> Matrix4<f32> {
    let rotate_x = Matrix4::from_angle_x(rotation[0]);
    let rotate_y = Matrix4::from_angle_y(rotation[1]);
    let rotate_z = Matrix4::from_angle_z(rotation[2]);
    rotate_z * rotate_y * rotate_x
}

#[must_use]
pub fn create_transforms(
    translation: [f32; 3],
    rotation: [f32; 3],
    scaling: [f32; 3],
) -> Matrix4<f32> {
    let trans = Matrix4::from_translation(translation.into());
    let rot = create_rotation(rotation.map(Rad));
    let scale = Matrix4::from_nonuniform_scale(scaling[0], scaling[1], scaling[2]);
    trans * rot * scale
}
