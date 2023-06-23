use webgpu_book::{RenderConfiguration, run_wgpu, WindowConfiguration};

pub fn run_example(title: &str, shader_source: &str) -> ! {
    run_wgpu(
        &WindowConfiguration { title },
        RenderConfiguration  {
            shader_source,
            vertices: 3,
            ..RenderConfiguration::default()
        },
    )
}
