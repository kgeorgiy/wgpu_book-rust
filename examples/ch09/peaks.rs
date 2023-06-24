use crate::common::run_simple_surface;

#[path = "../common/colormap.rs"]
mod colormap;
#[path = "../common/vertex_data.rs"]
mod vertex_data;
mod surface_data;
mod common;

pub fn peaks(x: f32, z: f32) -> f32 {
    3.0 * (1.0 - x) * (1.0 - x) * (-(x * x) - (z + 1.0) * (z + 1.0)).exp()
        - 10.0 * (x / 5.0 - x * x * x - z * z * z * z * z) * (-x * x - z * z).exp()
        - 1.0 / 3.0 * (-(x + 1.0) * (x + 1.0) - z * z).exp()
}

fn main() {
    run_simple_surface(
        "Ch. 9. Peaks", &peaks,
        -3.0, 3.0,
        -3.0, 3.0,
        51, 51,
        2.0, 0.0
    );
}
