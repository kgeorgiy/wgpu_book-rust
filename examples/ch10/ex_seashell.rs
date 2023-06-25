use std::f32::consts::PI;

use crate::common::common10::run_parametric_surface;
use crate::common::functions::seashell;

#[path = "../common/common.rs"]
mod common;

fn main() {
    run_parametric_surface(
        "Chapter 10. Seashell", &seashell,
        (0.0, 1.0, 200), (0.0, 2.0 * PI, 40), (2.0, 3.0, 2.0)
    );
}
