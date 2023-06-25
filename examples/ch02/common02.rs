use webgpu_book::{RenderConfiguration, run_wgpu_title};

pub fn run_example(title: &str, shader_source: &str) -> ! {
    run_wgpu_title(title, RenderConfiguration {
        shader_source: shader_source.to_string(),
        vertices: 3,
        ..RenderConfiguration::default()
    })
}
