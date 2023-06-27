use wgpu::{IndexFormat, PrimitiveTopology};

use webgpu_book::PipelineConfiguration;

use crate::global_common::CmdArgs;

#[path = "../common/global_common.rs"]
mod global_common;

fn main() {
    let primitive_type = CmdArgs::next_known(&[
        "triangle-strip",
        "point-list",
        "line-list",
        "line-strip",
        "triangle-list",
    ]);

    let (topology, strip_index_format) = match primitive_type.as_str() {
        "point-list" => (PrimitiveTopology::PointList, None),
        "line-list" => (PrimitiveTopology::LineList, None),
        "line-strip" => (PrimitiveTopology::LineStrip, Some(IndexFormat::Uint32)),
        "triangle-list" => (PrimitiveTopology::TriangleList, None),
        "triangle-strip" => (PrimitiveTopology::TriangleStrip, Some(IndexFormat::Uint32)),
        _ => panic!("Unknown type {primitive_type}"),
    };

    PipelineConfiguration::new(include_str!("topology.wgsl"))
        .with_vertex_count(6)
        .with_full_topology(topology, strip_index_format)
        .with_cull_mode(None)
        .run_title(format!("Chapter 4. Topology: {primitive_type}").as_str())
}
