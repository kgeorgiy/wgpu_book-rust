use crate::run::run_example;

mod run;

fn main() -> ! {
    run_example("ch02: First triangle", include_str!("first_triangle.wgsl"));
}
