#![allow(clippy::indexing_slicing)]

use core::f32::consts::PI;
use core::iter::zip;

use cgmath::{Array, ElementWise, InnerSpace, Point3, point3, Vector3};

use webgpu_book::{Configurator, func_box, PipelineConfiguration, VertexBufferInfo};
use webgpu_book::boxed::FuncBox;

use super::{Vertex, VertexC, VertexNCT};
use super::CmdArgs;
use super::colormap::{Colormap, ColormapInterpolator};
use super::functions::{breather, klein_bottle, peaks, seashell, sievert_enneper, sinc, sphere, torus, wellenkugel};

fn normalize_point(
    point: &Point3<f32>,
    (min, max): (&Point3<f32>, &Point3<f32>),
    scale: &Point3<f32>
) -> Point3<f32> {
    const ONES: Point3<f32> = point3(-1.0, -1.0, -1.0);
    (ONES + (point - min).div_element_wise(max - min) * 2.0).mul_element_wise(*scale)
}

fn create_triangle_quad<V: From<VertexNCT> + Copy>(
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
    fn quads<V>(
        &self,
        colormap: &Colormap,
        global_uv: bool,
    ) -> Quads<V> where V: From<VertexNCT> + Copy {
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

        let color = &colormap.interpolator((
            normalize_point(&point3(0.0, min_y, 0.0), min_max1, &scale_p).y,
            normalize_point(&point3(0.0, max_y, 0.0), min_max1, &scale_p).y
        ));

        let (td_u, td_v) = if global_uv {
            (1.0 / (nu as f32 - 1.0), 1.0 / (nv as f32 - 1.0))
        } else {
            (1.0, 1.0)
        };

        let pts = &points;
        (0..nu - 1).flat_map(|i| {
            let u = td_u * i as f32;
            let u_1 = td_u * (i + 1) as f32;
            (0.. nv - 1).map(move |j| {
                let v = td_v * j as f32;
                let v_1 = td_v * (j + 1) as f32;

                let verts = [
                    (pts[i][j], (u, v)),
                    (pts[i + 1][j], (u_1, v)),
                    (pts[i + 1][j + 1], (u_1, v_1)),
                    (pts[i][j + 1], (u, v_1)),
                ];

                let normal = (verts[3].0 - verts[0].0).cross(verts[1].0 - verts[0].0).normalize();
                verts
                    .map(|(point, uv)| V::from(VertexNCT::new(
                        point,
                        normal,
                        color.interpolate(point.y),
                        uv
                    )))
            })
        }).into()
    }
}

#[must_use]
pub struct Surface<'a> {
    pub(crate) name: &'a str,
    data: fn() -> SurfaceData
}

impl<'a> Surface<'a> {
    #[must_use]
    pub fn name(&self) -> &'a str {
        self.name
    }
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

    pub fn by_name(name: &str) -> &'static Self {
        Self::SURFACES.iter()
            .find(|surface| surface.name == name)
            .unwrap_or_else(||  panic!("Unknown surface type {name}"))
    }

    pub fn read_args_surface() -> &'static Self {
        let known = Self::SURFACES.iter()
            .map(|surface| surface.name)
            .collect::<Vec<_>>();
        Self::by_name(CmdArgs::next_known("Surface type", &known).as_str())
    }

    pub fn read_args_triangles(colormap: &Colormap, global_uv: bool) -> (String, Triangles<VertexNCT>)     {
        let surface = Surface::read_args_surface();
        (surface.name.to_owned(), surface.triangles(colormap, global_uv))
    }
}

impl<'a> Surface<'a> {
    pub fn quads(&self, colormap: &Colormap, global_uv: bool) -> Quads<VertexNCT> {
        (self.data)().quads(colormap, global_uv)
    }

    pub fn triangles(&self, colormap: &Colormap, global_uv: bool) -> Triangles<VertexNCT> {
        self.quads(colormap, global_uv).triangles()
    }

    pub fn edges(&self, color: Point3<f32>) -> Edges<VertexNCT> {
        self.quads(&Colormap::fixed(color), true).into()
    }

    #[allow(clippy::unused_self)]
    pub fn axes(&self, scale: f32) -> Edges<VertexC> {
        [
            point3(1.0, 0.0, 0.0),
            point3(0.0, 1.0, 0.0),
            point3(0.0, 0.0, 1.0),
        ].into_iter().map(|v| {
            let color = Point3::from_value(0.5).mul_element_wise(v);
            [
                VertexC::new(Point3::from_value(-scale).mul_element_wise(v), color),
                VertexC::new(Point3::from_value(scale).mul_element_wise(v), color + Vector3::from_value(0.5)),
            ]
        }).into()
    }
}


// Mesh

#[derive(Clone)]
#[must_use]
pub struct Mesh<T, const L: usize> {
    mesh: Vec<[T; L]>,
}

impl<V, const L: usize> Mesh<V, L> {
    pub fn join<T: Iterator<Item=Mesh<V, L>>>(meshes: T) -> Self {
        meshes.flat_map(|mesh| mesh.mesh).into()
    }

    pub fn iter(&self) -> core::slice::Iter<'_, [V; L]> {
        self.mesh.iter()
    }

    pub fn map<U, F: Fn(V) -> U>(self, f: F) -> Mesh<U, L> {
        self.into_iter().map(|vertices| vertices.map(&f)).into()
    }

    pub fn cast<U>(self) -> Mesh<U, L> where V: Into<U> {
        self.map(V::into)
    }

    #[must_use]
    pub fn vertices(self) -> Vec<V> {
        self.into()
    }
}

impl<V, const L: usize> IntoIterator for Mesh<V, L>{
    type Item = [V; L];
    type IntoIter = std::vec::IntoIter<[V; L]>;

    fn into_iter(self) -> Self::IntoIter {
        self.mesh.into_iter()
    }
}

impl<V, const L: usize, U> From<Mesh<V, L>> for Vec<U> where V: Into<U> {
    #[must_use]
    fn from(mesh: Mesh<V, L>) -> Self {
        mesh.mesh.into_iter().flatten().map(V::into).collect()
    }
}

impl<V, const L: usize, T: Iterator<Item=[V; L]>> From<T> for Mesh<V, L> {
    fn from(mesh: T) -> Self {
        Mesh { mesh: mesh.collect() }
    }
}


// Quads
pub type Quads<V> = Mesh<V, 4>;

impl<V: Copy> Quads<V> {
    pub fn triangles(self) -> Triangles<V> {
        Triangles::from(self)
    }

    pub fn edges(self) -> Edges<V> {
        Edges::from(self)
    }
}

// Edges

pub type Edges<V> = Mesh<V, 2>;

impl<V: VertexBufferInfo> Edges<V> {
    pub(crate) fn conf_shader(self, shader_source: &str) -> Configurator<PipelineConfiguration> {
        let shader_string = shader_source.to_owned();
        func_box!(move |config: PipelineConfiguration| config
            .with_shader(shader_string.as_str())
            .with_vertices(self.vertices())
            .with_topology(wgpu::PrimitiveTopology::LineList))
    }
}

impl Mesh<Vertex, 2> {
    pub fn into_config(self) -> PipelineConfiguration {
        PipelineConfiguration::new("").with(self.conf_shader(include_str!("../ch06/line3d.wgsl")))
    }
}

impl<V, U> From<Quads<V>> for Edges<U> where U: Copy, V: Into<U> {
    fn from(quads: Quads<V>) -> Self {
        quads.cast()
            .mesh.into_iter()
            .flat_map(|[v0, v1, _, v3]| [[v0, v1], [v0, v3]])
            .into()
    }
}

impl<V, U> From<Triangles<V>> for Edges<U> where U: Copy, V: Into<U> {
    fn from(triangles: Triangles<V>) -> Self {
        triangles.cast()
            .mesh.into_iter()
            .flat_map(|[v0, v1, v2]| [[v0, v1], [v1, v2], [v2, v0]])
            .into()
    }
}

//
// Triangles
pub type Triangles<V> = Mesh<V, 3>;

impl<V, U> From<Quads<V>> for Triangles<U> where U: Copy, V: Into<U> {
    fn from(quads: Quads<V>) -> Self {
        quads.cast()
            .mesh.into_iter()
            .flat_map(|[v0, v1, v2, v3]| [[v0, v3, v1], [v1, v3, v2]])
            .into()
    }
}
