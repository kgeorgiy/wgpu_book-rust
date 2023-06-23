use wgpu::{IndexFormat, PrimitiveTopology};

use webgpu_book::{run_wgpu, RenderConfiguration, WindowConfiguration};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let primitive_type: &str = if args.len() > 1 {
        &args[1]
    } else {
        "point-list"
    };

    let (topology, strip_index_format) = match primitive_type {
        "point-list" => (PrimitiveTopology::PointList, None),
        "line-list" => (PrimitiveTopology::LineList, None),
        "line-strip" => (PrimitiveTopology::LineStrip, Some(IndexFormat::Uint32)),
        "triangle-list" => (PrimitiveTopology::TriangleList, None),
        "triangle-strip" => (PrimitiveTopology::TriangleStrip, Some(IndexFormat::Uint32)),
        _ => panic!("Unknown type {}", primitive_type),
    };

    run_wgpu::<()>(
        &WindowConfiguration {
            title: &format!("Ch4. Topology: {}", primitive_type),
        },
        RenderConfiguration {
            shader_source: include_str!("topology.wgsl"),
            vertices: 6,
            topology,
            strip_index_format,
            ..RenderConfiguration::default()
        },
    )
}
