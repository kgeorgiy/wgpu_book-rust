use crate::common::run_example;
use crate::common::colormap::Colormap;
use crate::common::surface_data::Surface;

mod common;

fn main() {
    let colormap = Colormap::by_name("jet");
    let (name, vertices) = Surface::read_args_surface_vertices(&colormap, true);
    run_example(format!("Chapter 10. {name}").as_str(), &vertices);
}
