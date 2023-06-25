use crate::common::run_simple_surface;

#[path = "../common/colormap.rs"]
mod colormap;
#[path = "../common/vertex_data.rs"]
mod vertex_data;
mod surface_data;
mod common;

pub fn sinc(x: f32, y: f32) -> f32 {
    let r = (x * x + y * y).sqrt();
    if r == 0.0 { 1.0 } else { r.sin() / r }
}

fn main() {
    run_simple_surface(
        "Ch. 9. Sinc", &sinc,
        (-8.0, 8.0, 30), (-8.0, 8.0, 30), 2.0
    );
}
