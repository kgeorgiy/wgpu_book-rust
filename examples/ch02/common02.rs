use webgpu_book::RenderConfiguration;

pub fn run_example(title: &str, shader_source: &str) -> ! {
    RenderConfiguration::<0> {
        shader_source: shader_source.to_string(),
        vertices: 3,
        ..RenderConfiguration::default()
    }.run_title(title)
}
