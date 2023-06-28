use cgmath::{Matrix4, SquareMatrix, vec3};

use webgpu_book::{PipelineConfiguration, TextureInfo};
use crate::common::colormap::Colormap;

use crate::common::light::{ProtoUniforms, TwoSideLightAux};
use crate::common::surface_data::surface_vertices;

pub use self::global_common::*;

#[path = "../common/global_common.rs"]
mod global_common;

#[must_use] pub fn example_models<const T: usize>(vertices: &[VertexNCT], models: [Matrix4<f32>; T], instances: bool) -> PipelineConfiguration {
    let texture_file = CmdArgs::next("whitesquare2");
    let is_two_side = CmdArgs::next_bool("Is two side", false);

    let light_aux = TwoSideLightAux::new(is_two_side);

    ProtoUniforms::example_models(
        include_str!("instances.wgsl"),
        vertices,
        light_aux,
        models,
        instances,
    )

        .with_textures([TextureInfo::repeated(format!("examples/ch11/assets/{texture_file}.png"))])
}

#[allow(dead_code, clippy::indexing_slicing)]
#[must_use] pub fn multi_pipeline(surface_name: &str, instances: bool) -> PipelineConfiguration {
    const ROWS: usize = 7;
    const COLS: usize = 5;

    let colormap = Colormap::by_name("jet");
    let vertices = surface_vertices(surface_name, &colormap, false);

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

    example_models(&vertices, models, instances)
}
