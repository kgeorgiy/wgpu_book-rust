use webgpu_book::RenderConfiguration;

use crate::common::multi_pipeline;

mod common;
#[path = "../ch07/ex_torus.rs"]
mod ex_torus;

fn main() -> ! {
    let wellen = multi_pipeline("wellen", true);
    let torus = ex_torus::pipeline();
    RenderConfiguration { pipelines: vec![wellen, torus]}
        .run_title("Chapter 12. Multiple pipelines")
}
