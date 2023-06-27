use webgpu_book::PipelineConfiguration;

fn main() -> ! {
    PipelineConfiguration::new(include_str!("triangle_vertex_color.wgsl"))
        .with_vertex_count(3)
        .run_title("Chapter 02. Triangle vertex color");
}
