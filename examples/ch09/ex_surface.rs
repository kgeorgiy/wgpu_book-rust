use crate::common::{CmdArgs, run_surface};
use crate::common::colormap::Colormap;
use crate::common::surface_data::Surface;

mod common;

fn main() {
    let colormap = &Colormap::by_name(CmdArgs::next("jet").as_str());
    let (name, vertices) = Surface::read_args_surface_vertices(colormap, true);
    run_surface(format!("Chapter 09. {name}").as_str(), &vertices);
}
