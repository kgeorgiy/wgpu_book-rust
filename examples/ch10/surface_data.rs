use cgmath::{ElementWise, InnerSpace, point2, Point2, Point3, point3, vec3, Vector3, Vector4};

use crate::common::common10::Vertex;

fn normalize_point(
    point: &Point3<f32>,
    (min, max): (&Point3<f32>, &Point3<f32>),
    scale: &Point3<f32>,
    uv: Point2<f32>,
) -> Vertex {
    let normalized = (point - min).div_element_wise(max - min);
    Vertex::new(
        (point3(-1.0, -1.0, -1.0) + normalized * 2.0).mul_element_wise(*scale),
        vec3(0.0, 0.0, 0.0),
        uv
    )
}

fn create_quad(vertices: [Vertex; 4]) -> Vec<Vertex> {
    let points: Vec<Vector3<f32>> = vertices.iter()
        .map(|v| <[f32; 4] as Into<Vector4<f32>>>::into(v.position).truncate())
        .collect();
    let normal = (points[2] - points[0]).cross(points[3] - points[1]).normalize();
    let vs: Vec<Vertex> = vertices.iter()
        .map(|v| Vertex { normal: normal.extend(0.0).into(), ..*v})
        .collect();
    vec![vs[0], vs[1], vs[2], vs[2], vs[3], vs[0]]
}

pub fn simple_surface_data(
    f: &dyn Fn(f32, f32) -> f32,
    min_max_n_x: (f32, f32, usize),
    min_max_n_z: (f32, f32, usize),
    scale: f32,
) -> Vec<Vertex> {
    parametric_surface_data(
        &|x, z| point3(x, f(x, z), z),
        min_max_n_x,
        min_max_n_z,
        (scale, scale, scale),
    )
}

fn min_max(min_max: &mut (f32, f32), value: f32) {
    *min_max = (min_max.0.min(value), min_max.1.max(value));
}

pub fn parametric_surface_data(
    f: &dyn Fn(f32, f32) -> Point3<f32>,
    (min_u, max_u, nu): (f32, f32, usize),
    (min_v, max_v, nv): (f32, f32, usize),
    scale: (f32, f32, f32),
) -> Vec<Vertex> {
    let du = (max_u - min_u) / (nu as f32 - 1.0);
    let dv = (max_v - min_v) / (nv as f32 - 1.0);

    let mut min_max_x: (f32, f32) = (0.0, 0.0);
    let mut min_max_y: (f32, f32) = (0.0, 0.0);
    let mut min_max_z: (f32, f32) = (0.0, 0.0);

    let mut points: Vec<Vec<Point3<f32>>> = vec![vec![point3(0.0, 0.0, 0.0); nv]; nu];
    for i in 0..nu {
        let u = min_u + du * i as f32;
        for j in 0..nv {
            let v = min_v + dv * j as f32;
            let point = f(u, v);
            min_max(&mut min_max_x, point.x);
            min_max(&mut min_max_y, point.y);
            min_max(&mut min_max_z, point.z);
            points[i][j] = point;
        }
    }

    let (min_x, max_x) = min_max_x;
    let (min_y, max_y) = min_max_y;
    let (min_z, max_z) = min_max_z;

    let min = point3(min_x, min_y, min_z);
    let max = point3(max_x, max_y, max_z);
    let min_max = (&min, &max);

    let scale = Point3::from(scale);
    let mut vertices: Vec<Vec<Vertex>> = vec![vec![Vertex::new(point3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0), point2(0.0, 0.0)); nv]; nu];
    let du = 1.0 / (nu as f32 - 1.0);
    let dv = 1.0 / (nv as f32 - 1.0);
    for i in 0..nu {
        let u = du * i as f32;
        for j in 0..nv {
            let v = dv * j as f32;
            vertices[i][j] = normalize_point(&points[i][j], min_max, &scale, (u, v).into());
        }
    }

    let mut result: Vec<Vertex> = Vec::with_capacity(4 * (nu - 1) * (nv - 1));
    for i in 0..nu - 1 {
        for j in 0.. nv - 1 {
            let v0 = vertices[i][j];
            let v1 = vertices[i][j + 1];
            let v2 = vertices[i + 1][j + 1];
            let v3 = vertices[i + 1][j];
            result.append(&mut create_quad([v0, v1, v2, v3]));
        }
    }
    result
}
