use crate::common::common09::run_simple_surface;
use crate::common::functions::peaks;

#[path = "../common/common.rs"]
mod common;

fn main() {
    run_simple_surface(
        "Chapter 09. Peaks", &peaks,
        (-3.0, 3.0, 51), (-3.0, 3.0, 51), 2.0
    );
}
