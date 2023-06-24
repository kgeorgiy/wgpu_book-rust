use bytemuck::{Pod, Zeroable};
use cgmath::{ElementWise, InnerSpace, Point3, point3, Vector3};
use wgpu::VertexAttribute;
use webgpu_book::VertexBufferInfo;

use crate::colormap::{Colormap, ColormapInterpolator};

fn normalize_point(pt: &Point3<f32>, min: &Point3<f32>, max: &Point3<f32>, scale: f32) -> Point3<f32> {
    (point3(-1.0, -1.0, -1.0) + (pt - min).div_element_wise(max - min) * 2.0) * scale
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
    min_x: f32, max_x: f32,
    min_z: f32, max_z: f32,
    nx: usize, nz: usize,
    scale_xz: f32, scale_y: f32,
) -> Vec<Vertex> {
    parametric_surface_data(
        &|x, z| point3(x, f(x, z), z), colormap,
        min_x, max_x,
        min_z, max_z,
        nx, nz,
        min_x, max_x,
        min_z, max_z,
        scale_xz, scale_y,
    )
}

pub fn parametric_surface_data(
    f: &dyn Fn(f32, f32) -> Point3<f32>,
    colormap: &Colormap,
    min_u: f32, max_u: f32,
    min_v: f32, max_v: f32,
    nu: usize, nv: usize,
    min_x: f32, max_x: f32,
    min_z: f32, max_z: f32,
    scale_xz: f32, scale_y: f32,
) -> Vec<Vertex> {
    let du = (max_u - min_u) / (nu as f32 - 1.0);
    let dv = (max_v - min_v) / (nv as f32 - 1.0);

    let mut min_y: f32 = 0.0;
    let mut max_y: f32 = 0.0;
    let mut points: Vec<Vec<Point3<f32>>> = vec![vec![point3(0.0, 0.0, 0.0); nv]; nu];

    for i in 0..nu {
        let u = min_u + du * i as f32;
        for j in 0..nv {
            let v = min_v + dv * j as f32;
            let point = f(u, v);
            min_y = min_y.min(point.y);
            max_y = max_y.max(point.y);
            points[i][j] = point;
        }
    }

    let min = point3(min_x, min_y - scale_y * (max_y - min_y), min_z);
    let max = point3(max_x, max_y - scale_y * (max_y - max_y), max_z);
    for i in 0..nu {
        for j in 0..nv {
            points[i][j] = normalize_point(&points[i][j], &min, &max, scale_xz);
        }
    }

    let color = colormap.interpolator(
        normalize_point(&point3(0.0, min_y, 0.0), &min, &max, scale_xz).y,
        normalize_point(&point3(0.0, max_y, 0.0), &min, &max, scale_xz).y
    );

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
