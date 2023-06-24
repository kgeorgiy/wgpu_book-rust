use std::f32::consts::PI;

use cgmath::{Point3, point3};

use crate::common::run_parametric_surface;

#[path = "../common/colormap.rs"]
mod colormap;
#[path = "../common/vertex_data.rs"]
mod vertex_data;
mod surface_data;
mod common;

pub fn klein_bottle(u: f32, v: f32) -> Point3<f32> {
    let (sin_u, cos_u) = u.sin_cos();
    let (sin_v, cos_v) = v.sin_cos();
    let cos_sin = cos_v * sin_u;

    let x = 2.0 / 15.0 * (3.0 + 5.0 * cos_u * sin_u) * sin_v;
    let y = -1.0 / 15.0 * sin_u * (
        3.0 * cos_v - 3.0 * cos_u.powf(2.0) * cos_v
            - 48.0 * cos_u.powf(4.0) * cos_v
            + 48.0 * cos_u.powf(6.0) * cos_v
            - 60.0 * sin_u
            + 5.0 * cos_u * cos_sin
            - 5.0 * cos_u.powf(3.0) * cos_sin
            - 80.0 * cos_u.powf(5.0) * cos_sin
            + 80.0 * cos_u.powf(7.0) * cos_sin
    );
    let z = -2.0 / 15.0 * cos_u * (
        3.0 * cos_v - 30.0 * sin_u
            + 90.0 * cos_u.powf(4.0) * sin_u
            - 60.0 * cos_u.powf(6.0) * sin_u
            + 5.0 * cos_u * cos_v * sin_u
    );
    point3(x, -z, -y)
}

fn main() {
    run_parametric_surface(
        "Ch. 9. Klein bottle", &klein_bottle,
        0.0, PI, 0.0, 2.0 * PI, 70, 30, -2.0, 2.0, -2.0, 3.0, 2.5, 0.0
    );
}
