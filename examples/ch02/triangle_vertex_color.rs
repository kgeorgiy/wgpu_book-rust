use crate::run::run_example;

mod run;

fn main() -> ! {
    run_example(
        "ch02: Triangle vertex color",
        include_str!("triangle_vertex_color.wgsl"),
    );
}
