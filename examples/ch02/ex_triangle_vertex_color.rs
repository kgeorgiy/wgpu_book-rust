use crate::common::run_example;

mod common;

fn main() -> ! {
    run_example(
        "ch02: Triangle vertex color",
        include_str!("triangle_vertex_color.wgsl"),
    );
}
