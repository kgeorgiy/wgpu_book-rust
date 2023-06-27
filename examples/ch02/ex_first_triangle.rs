use webgpu_book::PipelineConfiguration;

fn main() -> ! {
    PipelineConfiguration::new(include_str!("first_triangle.wgsl"))
        .with_vertex_count(3)
        .run_title("Chapter 02. First triangle");
}
