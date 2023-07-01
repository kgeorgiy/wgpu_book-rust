use crate::common::colormap::Colormap;
use crate::common::run_example;
use crate::common::surface_data::Surface;

mod common;

fn main() {
    let colormap = Colormap::by_name("jet");
    let (name, triangles) = Surface::read_args_triangles(&colormap, false);
    run_example(format!("Chapter 11. Surface {name}").as_str(), triangles);
}
