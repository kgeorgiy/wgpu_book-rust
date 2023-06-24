use crate::common::run_simple_surface;

#[path = "../common/colormap.rs"]
mod colormap;
#[path = "../common/vertex_data.rs"]
mod vertex_data;
mod surface_data;
mod common;

pub fn sinc(x: f32, z: f32) -> f32 {
    let r = (x * x + z * z).sqrt();
    if r == 0.0 { 1.0 } else { r.sin() / r }
}

fn main() {
    run_simple_surface(
        "Ch. 9. Sinc", &sinc,
        -8.0, 8.0,
        -8.0, 8.0,
        30, 30,
        2.0, 0.3
    );
}
