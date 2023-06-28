#![allow(clippy::module_name_repetitions, clippy::indexing_slicing)]

use core::f32::consts::PI;
use core::iter::zip;

use cgmath::{ElementWise, InnerSpace, Point3, point3};

use super::CmdArgs;
use super::colormap::{Colormap, ColormapInterpolator};
use super::functions::{breather, klein_bottle, peaks, seashell, sievert_enneper, sinc, sphere, torus, wellenkugel};

use super::VertexNCT;

fn normalize_point(
    point: &Point3<f32>,
    (min, max): (&Point3<f32>, &Point3<f32>),
    scale: &Point3<f32>
) -> Point3<f32> {
    const ONES: Point3<f32> = point3(-1.0, -1.0, -1.0);
    (ONES + (point - min).div_element_wise(max - min) * 2.0).mul_element_wise(*scale)
}

fn create_quad<V: From<VertexNCT> + Copy>(
    points: [Point3<f32>; 4],
    color: &ColormapInterpolator,
    uvs: [(f32, f32); 4]
) -> Vec<V> {
    let normal = (points[2] - points[0]).cross(points[3] - points[1]).normalize();
    let vs: Vec<V> = zip(points, uvs)
        .map(|(point, uv)| V::from(VertexNCT::new(
            point,
            normal,
            color.interpolate(point.y),
            uv
        )))
        .collect();
    vec![vs[0], vs[1], vs[2], vs[2], vs[3], vs[0]]
}

pub fn simple_surface_data<V: From<VertexNCT> + Copy>(
    f: &dyn Fn(f32, f32) -> f32,
    colormap: &Colormap,
    global_uv: bool,
    min_max_n_x: (f32, f32, usize),
    min_max_n_z: (f32, f32, usize),
    scale: f32,
) -> Vec<V> {
    parametric_surface_data(
        &|x, z| point3(x, f(x, z), z),
        colormap,
        global_uv,
        min_max_n_x,
        min_max_n_z,
        (scale, scale, scale),
    )
}

fn min_max(min_max: &mut (f32, f32), value: f32) {
    *min_max = (min_max.0.min(value), min_max.1.max(value));
}

pub fn parametric_surface_data<V: From<VertexNCT> + Copy>(
    f: &dyn Fn(f32, f32) -> Point3<f32>,
    colormap: &Colormap,
    global_uv: bool,
    (min_u, max_u, nu): (f32, f32, usize),
    (min_v, max_v, nv): (f32, f32, usize),
    scale: (f32, f32, f32),
) -> Vec<V> {
    let du = (max_u - min_u) / (nu as f32 - 1.0);
    let dv = (max_v - min_v) / (nv as f32 - 1.0);

    let mut min_max_x: (f32, f32) = (0.0, 0.0);
    let mut min_max_y: (f32, f32) = (0.0, 0.0);
    let mut min_max_z: (f32, f32) = (0.0, 0.0);

    let mut points: Vec<Vec<Point3<f32>>> = vec![vec![point3(0.0, 0.0, 0.0); nv]; nu];
    for (i, row) in points.iter_mut().enumerate() {
        let u = min_u + du * i as f32;
        for (j, point) in row.iter_mut().enumerate() {
            let v = min_v + dv * j as f32;
            *point = f(u, v);
            min_max(&mut min_max_x, point.x);
            min_max(&mut min_max_y, point.y);
            min_max(&mut min_max_z, point.z);
        }
    }

    let (min_x, max_x) = min_max_x;
    let (min_y, max_y) = min_max_y;
    let (min_z, max_z) = min_max_z;

    let min = point3(min_x, min_y, min_z);
    let max = point3(max_x, max_y, max_z);
    let min_max = (&min, &max);

    let scale_p = Point3::from(scale);

    for row in &mut points {
        for point in row.iter_mut() {
            *point = normalize_point(point, min_max, &scale_p);
        }
    }

    let color = colormap.interpolator((
        normalize_point(&point3(0.0, min_y, 0.0), min_max, &scale_p).y,
        normalize_point(&point3(0.0, max_y, 0.0), min_max, &scale_p).y
    ));

    let mut vertices: Vec<V> = Vec::with_capacity(4 * (nu - 1) * (nv - 1));
    let (td_u, td_v) = if global_uv {
        (1.0 / (nu as f32 - 1.0), 1.0 / (nv as f32 - 1.0))
    } else {
        (1.0, 1.0)
    };
    for i in 0..nu - 1 {
        let u = td_u * i as f32;
        let u_1 = td_u * (i + 1) as f32;
        for j in 0.. nv - 1 {
            let v = td_v * j as f32;
            let v_1 = td_v * (j + 1) as f32;
            let p0 = points[i][j];
            let p1 = points[i][j + 1];
            let p2 = points[i + 1][j + 1];
            let p3 = points[i + 1][j];
            vertices.append(&mut create_quad(
                [p0, p1, p2, p3],
                &color,
                [(u, v), (u, v_1), (u_1, v_1), (u_1, v)],
            ));
        }
    }
    vertices
}

pub fn parametric_mesh_data<V: From<VertexNCT> + Copy>(
    f: &dyn Fn(f32, f32) -> Point3<f32>,
    colormap: &Colormap,
    global_uv: bool,
    (min_u, max_u, nu): (f32, f32, usize),
    (min_v, max_v, nv): (f32, f32, usize),
    scale: (f32, f32, f32),
) -> Vec<V> {
    let du = (max_u - min_u) / (nu as f32 - 1.0);
    let dv = (max_v - min_v) / (nv as f32 - 1.0);

    let mut min_max_x: (f32, f32) = (0.0, 0.0);
    let mut min_max_y: (f32, f32) = (0.0, 0.0);
    let mut min_max_z: (f32, f32) = (0.0, 0.0);

    let mut points: Vec<Vec<Point3<f32>>> = vec![vec![point3(0.0, 0.0, 0.0); nv]; nu];
    for (i, row) in points.iter_mut().enumerate() {
        let u = min_u + du * i as f32;
        for (j, point) in row.iter_mut().enumerate() {
            let v = min_v + dv * j as f32;
            *point = f(u, v);
            min_max(&mut min_max_x, point.x);
            min_max(&mut min_max_y, point.y);
            min_max(&mut min_max_z, point.z);
        }
    }

    let (min_x, max_x) = min_max_x;
    let (min_y, max_y) = min_max_y;
    let (min_z, max_z) = min_max_z;

    let min = point3(min_x, min_y, min_z);
    let max = point3(max_x, max_y, max_z);
    let min_max = (&min, &max);

    let scale_p = Point3::from(scale);

    for row in &mut points {
        for point in row.iter_mut() {
            *point = normalize_point(point, min_max, &scale_p);
        }
    }

    let color = colormap.interpolator((
        normalize_point(&point3(0.0, min_y, 0.0), min_max, &scale_p).y,
        normalize_point(&point3(0.0, max_y, 0.0), min_max, &scale_p).y
    ));

    let mut vertices: Vec<V> = Vec::with_capacity(4 * (nu - 1) * (nv - 1));
    let (td_u, td_v) = if global_uv {
        (1.0 / (nu as f32 - 1.0), 1.0 / (nv as f32 - 1.0))
    } else {
        (1.0, 1.0)
    };
    for i in 0..nu - 1 {
        let u = td_u * i as f32;
        let u_1 = td_u * (i + 1) as f32;
        for j in 0.. nv - 1 {
            let v = td_v * j as f32;
            let v_1 = td_v * (j + 1) as f32;
            let p0 = points[i][j];
            let p1 = points[i][j + 1];
            let p2 = points[i + 1][j + 1];
            let p3 = points[i + 1][j];
            vertices.append(&mut create_quad(
                [p0, p1, p2, p3],
                &color,
                [(u, v), (u, v_1), (u_1, v_1), (u_1, v)],
            ));
        }
    }
    vertices
}



#[must_use] pub fn read_args_surface_vertices<V: From<VertexNCT> + Copy>(colormap: &Colormap, global_uv: bool) -> (String, Vec<V>) {
    let name = read_args_surface_name();
    let vertices = surface_vertices(name.as_str(), colormap, global_uv);
    (name, vertices)
}

#[must_use] pub fn read_args_surface_name() -> String {
    CmdArgs::next_known("Surface type", &[
        "sinc", "peaks",
        "klein", "wellen", "seashell", "sievert", "breather",
        "torus", "sphere",
    ])
}

#[allow(clippy::missing_panics_doc)]
#[must_use] pub fn surface_vertices<V: From<VertexNCT> + Copy>(kind: &str, colormap: &Colormap, global_uv: bool) -> Vec<V> {
    let vertices: Vec<V> = match kind {
        "sinc" => simple_surface_data(
            &sinc, colormap, global_uv,
            (-8.0, 8.0, 30), (-8.0, 8.0, 30), 2.0,
        ),
        "peaks" => simple_surface_data(
            &peaks, colormap, global_uv,
            (-3.0, 3.0, 51), (-3.0, 3.0, 51), 2.0,
        ),
        "klein" => parametric_surface_data(
            &klein_bottle, colormap, global_uv,
            (0.0, PI, 70), (0.0, 2.0 * PI, 30), (1.0, 2.0, 2.0),
        ),
        "wellen" => parametric_surface_data(
            &wellenkugel, colormap, global_uv,
            (0.0, 14.5, 100), (0.0, 1.5 * PI, 50), (2.0, 1.5, 2.0),
        ),
        "seashell" => parametric_surface_data(
            &seashell, colormap, global_uv,
            (0.0, 1.0, 200), (0.0, 2.0 * PI, 40), (2.0, 3.0, 2.0),
        ),
        "sievert" => parametric_surface_data(
            &sievert_enneper, colormap, global_uv,
            (-PI / 2.001, PI / 2.001, 60), (0.00001, PI, 200), (20.0, 2.0, 2.0),
        ),
        "breather" => parametric_surface_data(
            &breather, colormap, global_uv,
            (-14.0, 14.0, 200), (-12.0 * PI, 12.0 * PI, 200), (3.0, 2.0, 2.0)
        ),
        "sphere" => parametric_surface_data(
            &sphere, colormap, global_uv,
            (-PI, PI, 20), (-PI / 2.0, PI / 2.0, 10), (2.0, 2.0, 2.0)
        ),
        "torus" => parametric_surface_data(
            &torus, colormap, global_uv,
            (-PI, PI, 40), (-PI, PI, 15), (2.0, 0.4, 2.0)
        ),
        _ => panic!("Unknown chart type"),
    };
    vertices
}
