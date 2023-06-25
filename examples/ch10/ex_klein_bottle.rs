use std::f32::consts::PI;

use crate::common::common10::run_parametric_surface;
use crate::common::functions::klein_bottle;

#[path = "../common/common.rs"]
mod common;

fn main() {
    run_parametric_surface(
        "Chapter 10. Klein bottle", &klein_bottle,
        (0.0, PI, 70), (0.0, 2.0 * PI, 30), (1.0, 2.0, 2.0)
    );
}
