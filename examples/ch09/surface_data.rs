use bytemuck::{Pod, Zeroable};
use cgmath::{ElementWise, InnerSpace, Point3, point3, Vector3};
use wgpu::VertexAttribute;

use webgpu_book::VertexBufferInfo;

use crate::colormap::{Colormap, ColormapInterpolator};

fn normalize_point(pt: &Point3<f32>, (min, max): (&Point3<f32>, &Point3<f32>), scale: &Point3<f32>) -> Point3<f32> {
    (point3(-1.0, -1.0, -1.0) + (pt - min).div_element_wise(max - min) * 2.0).mul_element_wise(*scale)
}

fn create_quad(points: [Point3<f32>; 4], color: &ColormapInterpolator) -> Vec<Vertex> {
    let normal = (points[2] - points[0]).cross(points[3] - points[1]).normalize();
    let vs: Vec<Vertex> = points.iter()
        .map(|point| Vertex::new(*point, normal, color.interpolate(point.y)))
        .collect();
    vec![vs[0], vs[1], vs[2], vs[2], vs[3], vs[0]]
}

pub fn simple_surface_data(
    f: &dyn Fn(f32, f32) -> f32,
    colormap: &Colormap,
    min_max_n_x: (f32, f32, usize),
    min_max_n_z: (f32, f32, usize),
    scale: f32,
) -> Vec<Vertex> {
    parametric_surface_data(
        &|x, z| point3(x, f(x, z), z), colormap,
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
    colormap: &Colormap,
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
    for i in 0..nu {
        for j in 0..nv {
            points[i][j] = normalize_point(&points[i][j], min_max, &scale);
        }
    }

    let color = colormap.interpolator((
        normalize_point(&point3(0.0, min_y, 0.0), min_max, &scale).y,
        normalize_point(&point3(0.0, max_y, 0.0), min_max, &scale).y
    ));

    let mut vertices: Vec<Vertex> = Vec::with_capacity(4 * (nu - 1) * (nv - 1));
    for i in 0..nu - 1 {
        for j in 0.. nv - 1 {
            let p0 = points[i][j];
            let p1 = points[i][j + 1];
            let p2 = points[i + 1][j + 1];
            let p3 = points[i + 1][j];
            vertices.append(&mut create_quad([p0, p1, p2, p3], &color));
        }
    }
    vertices
}


// Vertex

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 4],
    pub normal: [f32; 4],
    pub color: [f32; 4],
}

impl Vertex {
    pub fn new(position: Point3<f32>, normal: Vector3<f32>, color: Point3<f32>) -> Self {
        Self {
            position: position.to_homogeneous().into(),
            normal: normal.normalize().extend(0.0).into(),
            color: color.to_homogeneous().into(),
        }
    }
}

impl VertexBufferInfo for Vertex {
    const ATTRIBUTES: &'static [VertexAttribute] = &wgpu::vertex_attr_array![0=>Float32x4, 1=>Float32x4, 2=>Float32x4];
}
