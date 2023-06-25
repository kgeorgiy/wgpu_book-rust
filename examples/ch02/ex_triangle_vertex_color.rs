use crate::common02::run_example;

mod common02;

fn main() -> ! {
    run_example("Chapter 02. Triangle vertex color", include_str!("triangle_vertex_color.wgsl"));
}
