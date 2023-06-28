



use crate::common::{multi_pipeline};
use crate::common::surface_data::{read_args_surface_name};

pub mod common;

#[allow(dead_code)]
fn main() {
    let name = read_args_surface_name();
    multi_pipeline(name.as_str(), false)
        .run_title(format!("Chapter 12. Multiple bindings ({name})").as_str());
}

