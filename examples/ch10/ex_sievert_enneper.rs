use std::f32::consts::PI;

use crate::common::common10::run_parametric_surface;
use crate::common::functions::sievert_enneper;

#[path = "../common/common.rs"]
mod common;

fn main() {
    run_parametric_surface(
        "Chapter 10. Sievert-Enneper", &sievert_enneper,
        (-PI / 2.001, PI / 2.001, 60), (0.00001, PI, 200), (20.0, 2.0, 2.0)
    );
}
