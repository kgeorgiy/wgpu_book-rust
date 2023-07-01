use cgmath::{Matrix4, SquareMatrix, vec3};

use crate::common::{example_models, VertexNCT};
use crate::common::colormap::Colormap;
use crate::common::surface_data::{Mesh, Surface};

mod common;

fn main() {
    let colormap = &Colormap::by_name("jet");
    let names =   [
        ["sinc", "peaks", "klein"],
        ["wellen", "seashell", "sievert"],
        ["breather", "torus", "sphere"],
    ];
    let scale = Matrix4::from_scale(0.3);
    let triangles = Mesh::join(names.iter().enumerate()
        .flat_map(move |(i, row)| row.iter().enumerate()
            .map(move |(j, name)| {
                let translation = vec3((i as f32 - 1.0) * 2.0, (j as f32 - 1.0) * 2.0, 0.0);
                let transform = Matrix4::from_translation(translation) * scale;
                Surface::by_name(name)
                    .triangles(colormap, false)
                    .map(move |vertex: VertexNCT| vertex.transform(transform))
            })
        ));

    example_models(triangles, [Matrix4::identity()], true)
        .run_title("Chapter 12. Merged vertices");
}
