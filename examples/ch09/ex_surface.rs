use std::f32::consts::PI;

use crate::common::{CmdArgs, run_parametric_surface};
use crate::common::functions::*;
use crate::common::run_simple_surface;

mod common;

fn main() {
    let kind = CmdArgs::next_known(&["sinc", "peaks", "klein", "wellen", "seashell", "sievert", "breather"]);
    match kind.as_str() {
        "sinc" => run_simple_surface(
            "Chapter 9. Sinc", &sinc,
            (-8.0, 8.0, 30), (-8.0, 8.0, 30), 2.0,
        ),
        "peaks" => run_simple_surface(
            "Chapter 09. Peaks", &peaks,
            (-3.0, 3.0, 51), (-3.0, 3.0, 51), 2.0,
        ),
        "klein" => run_parametric_surface(
            "Chapter 09. Klein bottle", &klein_bottle,
            (0.0, PI, 70), (0.0, 2.0 * PI, 30), (1.0, 2.0, 2.0),
        ),
        "wellen" => run_parametric_surface(
            "Chapter 09. Wellenkugel", &wellenkugel,
            (0.0, 14.5, 100), (0.0, 1.5 * PI, 50), (2.0, 1.5, 2.0),
        ),
        "seashell" => run_parametric_surface(
            "Chapter 09. Seashell", &seashell,
            (0.0, 1.0, 200), (0.0, 2.0 * PI, 40), (2.0, 3.0, 2.0),
        ),
        "sievert" => run_parametric_surface(
            "Chapter 09. Sievert-Enneper", &sievert_enneper,
            (-PI / 2.001, PI / 2.001, 60), (0.00001, PI, 200), (20.0, 2.0, 2.0),
        ),
        "breather" => run_parametric_surface(
            "Chapter 09. Breather", &breather,
            (-14.0, 14.0, 200), (-12.0 * PI, 12.0 * PI, 200), (3.0, 2.0, 2.0)
        ),
        _ => println!("Unknown chart type"),
    }
}
