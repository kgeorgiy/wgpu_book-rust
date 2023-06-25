use crate::common::run_example;

mod common;

fn main() -> ! {
    run_example("ch02: First triangle", include_str!("first_triangle.wgsl"));
}
