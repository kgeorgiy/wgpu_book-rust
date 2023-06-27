use crate::common::run_example;
use crate::common::colormap::Colormap;
use crate::common::surface_data::surface_vertices;

mod common;

fn main() {
    let colormap = Colormap::by_name("jet");
    let (kind, vertices) = surface_vertices(&colormap, false);
    run_example(format!("Chapter 11. {kind}").as_str(), &vertices);
}
