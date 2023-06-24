use std::f32::consts::PI;
use cgmath::{Point3, point3};

use crate::common::run_parametric_surface;

#[path = "../common/colormap.rs"]
mod colormap;
#[path = "../common/vertex_data.rs"]
mod vertex_data;
mod surface_data;
mod common;

pub fn seashell(u: f32, v: f32) -> Point3<f32> {
    let v2_cos2 = (v / 2.0).cos().powf(2.0);
    let v_sin = v.sin();
    let u_exp = u.exp();

    let (sin_u, cos_u) = (u * 6.0 * PI).sin_cos();
    let x = 2.0 * (u_exp - 1.0) * sin_u * v2_cos2;
    let y = 1.0 - (u * 2.0).exp() - v_sin + u_exp * v_sin;
    let z = 2.0 * (1.0 - u_exp) * cos_u * v2_cos2;
    point3(x, y, z)
}

fn main() {
    run_parametric_surface(
        "Ch. 9. Seashell", &seashell,
        0.0, 1.0, 0.0, 2.0 * PI, 200, 40, -4.0, 4.0, -4.0, 4.0, 2.5, 0.0
    );
}
