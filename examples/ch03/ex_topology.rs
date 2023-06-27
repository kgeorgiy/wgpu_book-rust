use wgpu::{IndexFormat, PrimitiveTopology};

use webgpu_book::RenderConfiguration;

use crate::global_common::{CmdArgs, Config};

#[path = "../common/global_common.rs"]
mod global_common;

fn main() {
    let primitive_type = CmdArgs::next("triangle-strip");

    let (topology, strip_index_format) = match primitive_type.as_str() {
        "point-list" => (PrimitiveTopology::PointList, None),
        "line-list" => (PrimitiveTopology::LineList, None),
        "line-strip" => (PrimitiveTopology::LineStrip, Some(IndexFormat::Uint32)),
        "triangle-list" => (PrimitiveTopology::TriangleList, None),
        "triangle-strip" => (PrimitiveTopology::TriangleStrip, Some(IndexFormat::Uint32)),
        _ => panic!("Unknown type {primitive_type}"),
    };

    let title = format!("Ch4. Topology: {primitive_type}");
    RenderConfiguration {
        vertices: 6,
        topology,
        strip_index_format,
        cull_mode: None,
        ..Config::with_shader(include_str!("topology.wgsl"))
    }.run_title(title.as_str())
}
