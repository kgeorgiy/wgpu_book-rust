use crate::common::run_example;
use crate::common::colormap::Colormap;
use crate::common::surface_data::read_args_surface_vertices;

mod common;

fn main() {
    let colormap = Colormap::by_name("jet");
    let (kind, vertices) = read_args_surface_vertices(&colormap, true);
    run_example(format!("Chapter 10. {kind}").as_str(), &vertices);
}