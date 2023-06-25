use std::f32::consts::PI;

use crate::common::common10::run_parametric_surface;
use crate::common::functions::breather;

#[path = "../common/common.rs"]
mod common;

fn main() {
    run_parametric_surface(
        "Chapter 10. Breather", &breather,
        (-14.0, 14.0, 200), (-12.0 * PI, 12.0 * PI, 200), (3.0, 2.0, 2.0)
    );
}
