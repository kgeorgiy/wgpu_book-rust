use webgpu_book::RenderConfiguration;

use ex_instances::common;
mod ex_instances;
#[path = "../ch07/ex_torus.rs"]
mod ex_torus;

fn main() -> ! {
    let wellen = ex_instances::pipeline("wellen");
    let torus = ex_torus::pipeline();
    RenderConfiguration { pipelines: vec![wellen, torus]}
        .run_title("Chapter 12. Multiple pipelines")
}


