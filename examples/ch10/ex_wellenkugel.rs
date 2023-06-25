use std::f32::consts::PI;

use crate::common::common10::run_parametric_surface;
use crate::common::functions::wellenkugel;

#[path = "../common/common.rs"]
mod common;

fn main() {
    run_parametric_surface(
        "Chapter 10. Wellenkugel", &wellenkugel,
        (0.0, 14.5, 100),  (0.0, 1.5 * PI, 50), (2.0, 1.5, 2.0)
    );
}
