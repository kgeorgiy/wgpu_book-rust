use webgpu_book::RenderPassConfiguration;

use crate::common::multi_pipeline;
use crate::common::surface_data::Surface;

mod common;
#[path = "../ch07/ex_torus.rs"]
mod ex_torus;

fn main() -> ! {
    let wellen = multi_pipeline(Surface::by_name("wellen"), true);
    let torus = ex_torus::pipeline();
    RenderPassConfiguration::new(vec![wellen, torus])
        .run_title("Chapter 12. Multiple pipelines")
}
