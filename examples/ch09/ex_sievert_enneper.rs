use std::f32::consts::PI;

use cgmath::{Point3, point3};

use crate::common::run_parametric_surface;

#[path = "../common/colormap.rs"]
mod colormap;
#[path = "../common/vertex_data.rs"]
mod vertex_data;
mod surface_data;
mod common;

pub fn sievert_enneper(u: f32, v: f32) -> Point3<f32> {
    const A: f32 = 1.0;

    let pu = -u / (1.0 + A).sqrt() + (u.tan() * (1.0 + A).sqrt()).atan();
    let (sin_v, cos_v) = v.sin_cos();
    let (sin_u, cos_u) = u.sin_cos();

    let auv = 2.0 / (1.0 + A - A * (sin_v * cos_u).powf(2.0));
    let ruv = auv * sin_v * ((1.0 + 1.0 / A) * (1.0 + A * sin_u * sin_u)).sqrt();
    let x = ((v / 2.0).tan().ln() + (1.0 + A) * auv * cos_v) / A.sqrt();

    point3(x, ruv * pu.cos(), ruv * pu.sin())
}

fn main() {
    run_parametric_surface(
        "Ch. 9. Sievert-Enneper", &sievert_enneper,
        (-PI / 2.001, PI / 2.001, 60), (0.00001, PI, 200), (20.0, 2.0, 2.0)
    );
}
