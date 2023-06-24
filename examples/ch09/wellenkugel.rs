use cgmath::{Point3, point3};

use crate::common::run_parametric_surface;

#[path = "../common/colormap.rs"]
mod colormap;
#[path = "../common/vertex_data.rs"]
mod vertex_data;
mod surface_data;
mod common;

pub fn wellenkugel(u: f32, v: f32) -> Point3<f32> {
    let (sin_v, cos_v) = v.sin_cos();
    let (sin_cos_u, cos_cos_u) = u.cos().sin_cos();
    point3(cos_cos_u * sin_v, sin_cos_u, cos_cos_u * cos_v) * u
}

fn main() {
    run_parametric_surface(
        "Ch. 9. Wellenkugel", &wellenkugel,
        0.0, 14.5,  0.0, 5.0, 100, 50, -10.0, 10.0, -10.0, 10.0, 1.5, 0.0
    );
}
