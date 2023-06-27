use cgmath::{Matrix4, SquareMatrix, vec3};
use webgpu_book::PipelineConfiguration;

use crate::common::colormap::Colormap;
use crate::common::example_models;
use crate::common::surface_data::{read_args_surface_name, surface_vertices};

#[path = "common.rs"]
pub mod common;

#[allow(clippy::indexing_slicing)]
#[must_use] pub fn pipeline(surface_name: &str) -> PipelineConfiguration {
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

    example_models(&vertices, models)
}

#[allow(dead_code)]
fn main() {
    let name = read_args_surface_name();
    pipeline(name.as_str())
        .run_title(format!("Chapter 12. Instances ({name})").as_str());
}

