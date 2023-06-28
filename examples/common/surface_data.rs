#![allow(clippy::module_name_repetitions, clippy::indexing_slicing)]

use core::f32::consts::PI;
use core::iter::zip;

use cgmath::{ElementWise, InnerSpace, Point3, point3};

use super::CmdArgs;
use super::colormap::{Colormap, ColormapInterpolator};
use super::functions::{breather, klein_bottle, peaks, seashell, sievert_enneper, sinc, sphere, torus, wellenkugel};
use super::{VertexC, VertexNCT};

fn normalize_point(
    point: &Point3<f32>,
    (min, max): (&Point3<f32>, &Point3<f32>),
    scale: &Point3<f32>
) -> Point3<f32> {
    const ONES: Point3<f32> = point3(-1.0, -1.0, -1.0);
    (ONES + (point - min).div_element_wise(max - min) * 2.0).mul_element_wise(*scale)
}

fn create_face_quad<V: From<VertexNCT> + Copy>(
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

fn create_wireframe_quad<V: From<VertexC> + Copy>(
    points: [Point3<f32>; 4],
    color: &ColormapInterpolator,
    uvs: [(f32, f32); 4]
) -> Vec<V> {
    let vertices = points
        .map(|point| VertexC::new(point, color.interpolate(point.y)))
        .map(V::from);
    let mut result = vec![vertices[1], vertices[2], vertices[2], vertices[3]];
    if uvs[0].0 == 0.0 {
        result.append(&mut vec![vertices[0], vertices[1]]);
    }
    if uvs[0].1 == 0.0 {
        result.append(&mut vec![vertices[0], vertices[3]]);
    }
    result
}


fn min_max(min_max: &mut (f32, f32), value: f32) {
    *min_max = (min_max.0.min(value), min_max.1.max(value));
}

struct SurfaceData {
    f: Box<dyn Fn(f32, f32) -> Point3<f32>>,
    min_max_n_u: (f32, f32, usize),
    min_max_n_v: (f32, f32, usize),
    scale: Point3<f32>,
}

type QuadF<V> = fn([Point3<f32>; 4], &ColormapInterpolator, [(f32, f32); 4]) -> Vec<V>;

impl SurfaceData {
    fn data<U, V>(
        &self,
        colormap: &Colormap,
        global_uv: bool,
        quad: QuadF<V>
    ) -> Vec<V> where V: From<U> + Copy {
        let (min_u, max_u, nu) = self.min_max_n_u;
        let (min_v, max_v, nv) = self.min_max_n_v;
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
                *point = (self.f)(u, v);
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
        let min_max1 = (&min, &max);

        let scale_p = self.scale;

        for row in &mut points {
            for point in row.iter_mut() {
                *point = normalize_point(point, min_max1, &scale_p);
            }
        }

        let color = colormap.interpolator((
            normalize_point(&point3(0.0, min_y, 0.0), min_max1, &scale_p).y,
            normalize_point(&point3(0.0, max_y, 0.0), min_max1, &scale_p).y
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
                vertices.append(&mut quad(
                    [p0, p1, p2, p3],
                    &color,
                    [(u, v), (u, v_1), (u_1, v_1), (u_1, v)],
                ));
            }
        }
        vertices
    }
}

pub struct Surface<'a> {
    pub name: &'a str,
    data: fn() -> SurfaceData
}

impl Surface<'static> {
    const SURFACES: [Surface<'static>; 9] = [
        Self::new("sinc", || Self::simple(&sinc, (-8.0, 8.0, 30), (-8.0, 8.0, 30), 2.0)),
        Self::new("peaks", || Self::simple(&peaks, (-3.0, 3.0, 51), (-3.0, 3.0, 51), 2.0)),
        Self::new("klein", || Self::parametric(&klein_bottle, (0.0, PI, 70), (0.0, 2.0 * PI, 30), (1.0, 2.0, 2.0))),
        Self::new("wellen", || Self::parametric(&wellenkugel, (0.0, 14.5, 100), (0.0, 1.5 * PI, 50), (2.0, 1.5, 2.0))),
        Self::new("seashell", || Self::parametric(&seashell, (0.0, 1.0, 200), (0.0, 2.0 * PI, 40), (2.0, 3.0, 2.0))),
        Self::new("sievert", || Self::parametric(&sievert_enneper, (-PI / 2.001, PI / 2.001, 60), (0.00001, PI, 200), (20.0, 2.0, 2.0))),
        Self::new("breather", || Self::parametric(&breather, (-14.0, 14.0, 200), (-12.0 * PI, 12.0 * PI, 200), (3.0, 2.0, 2.0))),
        Self::new("sphere", || Self::parametric(&sphere, (-PI, PI, 20), (-PI / 2.0, PI / 2.0, 10), (2.0, 2.0, 2.0))),
        Self::new("torus", || Self::parametric(&torus, (-PI, PI, 40), (-PI, PI, 15), (2.0, 0.4, 2.0))),
    ];

    const fn new(name: &str, data: fn() -> SurfaceData) -> Surface {
        Surface { name, data }
    }

    fn parametric(
        f: &'static dyn Fn(f32, f32) -> Point3<f32>,
        min_max_n_u: (f32, f32, usize),
        min_max_n_v: (f32, f32, usize),
        scale: (f32, f32, f32)
    ) -> SurfaceData {
        SurfaceData { f: Box::new(f), min_max_n_u, min_max_n_v, scale: scale.into() }
    }

    fn simple(
        f: &'static dyn Fn(f32, f32) -> f32,
        min_max_n_x: (f32, f32, usize),
        min_max_n_z: (f32, f32, usize),
        scale: f32,
    ) -> SurfaceData {
        let f3d = Box::new(|x, z| point3(x, f(x, z), z));
        SurfaceData { f: f3d, min_max_n_u: min_max_n_x, min_max_n_v: min_max_n_z, scale: point3(scale, scale, scale) }
    }

    #[must_use] pub fn by_name(name: &str) -> &'static Self {
        Self::SURFACES.iter()
            .find(|surface| surface.name == name)
            .unwrap_or_else(||  panic!("Unknown surface type {name}"))
    }

    #[must_use] pub fn read_args_surface() -> &'static Self {
        let known = Self::SURFACES.iter()
            .map(|surface| surface.name)
            .collect::<Vec<_>>();
        Self::by_name(CmdArgs::next_known("Surface type", &known).as_str())
    }

    #[must_use] pub fn read_args_surface_vertices<V: From<VertexNCT> + Copy>(colormap: &Colormap, global_uv: bool) -> (String, Vec<V>) {
        let surface = Surface::read_args_surface();
        let vertices = surface.surface_vertices(colormap, global_uv);
        (surface.name.to_owned(), vertices)
    }
}

impl<'a> Surface<'a> {
    #[must_use] pub fn surface_vertices<V>(&self, colormap: &Colormap, global_uv: bool)
        -> Vec<V> where V: From<VertexNCT> + Copy
    {
        (self.data)().data(colormap, global_uv, create_face_quad)
    }

    #[must_use] pub fn wireframe_vertices<V>(&self, color: Point3<f32>)
        -> Vec<V> where V: From<VertexC> + Copy
    {
        (self.data)().data(&Colormap::fixed(color), true, create_wireframe_quad)
    }

    #[must_use] pub fn axes_vertices(&self) -> Vec<VertexC> {
        let scale = (self.data)().scale * 1.25;
        vec![
            VertexC::new((-scale.x, 0.0, 0.0), (0.5, 0.0, 0.0)),
            VertexC::new(( scale.x, 0.0, 0.0), (1.0, 0.5, 0.5)),
            VertexC::new((0.0, -scale.y, 0.0), (0.0, 0.5, 0.0)),
            VertexC::new((0.0,  scale.y, 0.0), (0.5, 1.0, 0.5)),
            VertexC::new((0.0, 0.0, -scale.z), (0.0, 0.0, 0.5)),
            VertexC::new((0.0, 0.0,  scale.z), (0.5, 0.5, 1.0)),
        ]
    }
}
