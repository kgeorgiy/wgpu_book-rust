use crate::common::common10::run_simple_surface;
use crate::common::functions::sinc;

#[path="../common/common.rs"]
mod common;

fn main() {
    run_simple_surface(
        "Chapter 10. Sinc", &sinc,
        (-8.0, 8.0, 30), (-8.0, 8.0, 30), 2.0
    );
}
