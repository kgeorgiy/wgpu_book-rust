use crate::common::multi_pipeline;
use crate::common::surface_data::Surface;

pub mod common;

#[allow(dead_code)]
fn main() {
    let surface = Surface::read_args_surface();
    multi_pipeline(surface, true)
        .run_title(format!("Chapter 12. Multiple bindings ({})", surface.name).as_str());
}

