use cgmath::{Matrix4, SquareMatrix, vec3};

use webgpu_book::{PipelineConfiguration, TextureInfo};

use crate::common::colormap::Colormap;
use crate::common::light::{LightExamples, TwoSideLight};
use crate::common::surface_data::{Edges, Surface, Triangles};

pub use self::global_common::*;

#[path = "../common/global_common.rs"]
mod global_common;

pub fn example_models<const T: usize>(triangles: Triangles<VertexNCT>, models: [Matrix4<f32>; T], instances: bool) -> PipelineConfiguration {
    let texture_file = CmdArgs::next("whitesquare2");
    let is_two_side = CmdArgs::next_bool("Is two side", false);

    let light_aux = TwoSideLight::new(is_two_side);

    let shader_source = include_str!("instances.wgsl");
    PipelineConfiguration::new(shader_source)
        .with(LightExamples::models(light_aux, models, instances))
        .with_cull_mode(None)
        .with(LightExamples::read_args_wireframe(triangles))
        .with_textures([TextureInfo::repeated(format!("examples/ch11/assets/{texture_file}.png"))])
}

#[allow(dead_code, clippy::indexing_slicing)]
pub fn multi_pipeline(surface: &Surface, instances: bool) -> PipelineConfiguration {
    const ROWS: usize = 7;
    const COLS: usize = 5;

    let colormap = Colormap::by_name("jet");
    let vertices = Surface::triangles(surface, &colormap, false);

    let scale = 1.0 / (COLS - 1) as f32;
    let scale_m = Matrix4::from_scale(scale);

    let mut models = [Matrix4::identity(); ROWS * COLS];
    for r in 0..ROWS {
        for c in 0..COLS {
            let x = r as f32 - (COLS - 1) as f32 / 2.0;
            let y = c as f32 - (ROWS - 1) as f32 / 2.0;
            models[r * COLS + c] = Matrix4::from_translation(vec3(x, y, 0.0)) * scale_m;
        }
    }

    example_models(vertices, models, instances)
}

#[allow(dead_code)]
pub fn edges_pipeline(edges: Edges<VertexC>) -> PipelineConfiguration {
    PipelineConfiguration::new(include_str!("mesh.wgsl"))
        .with(TwoSideLight::read_args())
        .with(edges.vertices())
}
