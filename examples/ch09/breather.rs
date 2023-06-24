use std::f32::consts::PI;

use cgmath::{Point3, point3};

use crate::common::run_parametric_surface;

#[path = "../common/colormap.rs"]
mod colormap;
#[path = "../common/vertex_data.rs"]
mod vertex_data;
mod surface_data;
mod common;

pub fn breather(u: f32, v: f32) -> Point3<f32> {
    const A: f32 = 0.4; // where 0 < A < 1

    let aa1 = 1.0 - A * A;
    let aa1s = aa1.sqrt();
    let au_cosh = (A * u).cosh();
    let au_sinh = (A * u).sinh();

    let (av_sin, av_cos) = (aa1s * v).sin_cos();

    let de = A * (aa1 * au_cosh.powf(2.0) + (A * av_sin).powf(2.0));
    let x = -u * de / 2.0 + aa1 * au_cosh * au_sinh;
    let y = aa1s * au_cosh * (-aa1s * v.cos() * av_cos - v.sin() * av_sin);
    let z = aa1s * au_cosh * (-aa1s * v.sin() * av_cos + v.cos() * av_sin);

    point3(x, y, z) * 2.0 / de
}

fn main() {
    run_parametric_surface(
        "Ch. 9. Breather", &breather,
        -14.0, 14.0, -12.0 * PI, 12.0 * PI, 200, 200, -6.0, 6.0, -6.0, 6.0, 2.0, 0.0
    );
}
