#![allow(dead_code)]

use cgmath::{Angle, Deg, Point2, point2, Point3, point3, vec3, Vector3};
use super::surface_data::Mesh;

#[must_use] pub fn i8_as_f32<const R: usize, const C: usize>(values: [[i8; C]; R]) -> [[f32; C]; R] {
    values.map(|vertex| vertex.map(f32::from))
}

pub struct CubeFaceData {
    pub positions: [[i8; 3]; 36],
    pub colors: [[i8; 3]; 36],
    pub uvs: [[i8; 2]; 36],
    pub normals: [[i8; 3]; 36],
}

pub const FACE_COLORS_CUBE: CubeFaceData = CubeFaceData {
    positions: [
        // front (0, 0, 1)
        [-1, -1, 1], [1, -1, 1], [-1, 1, 1], [-1, 1, 1], [1, -1, 1], [1, 1, 1],
        // right (1, 0, 0)
        [1, -1, 1], [1, -1, -1], [1, 1, 1], [1, 1, 1], [1, -1, -1], [1, 1, -1],
        // back (0, 0, -1)
        [1, -1, -1], [-1, -1, -1], [1, 1, -1], [1, 1, -1], [-1, -1, -1], [-1, 1, -1],
        // left (-1, 0, 0)
        [-1, -1, -1], [-1, -1, 1], [-1, 1, -1], [-1, 1, -1], [-1, -1, 1], [-1, 1, 1],
        // top (0, 1, 0)
        [-1, 1, 1], [1, 1, 1], [-1, 1, -1], [-1, 1, -1], [1, 1, 1], [1, 1, -1],
        // bottom (0, -1, 0)
        [-1, -1, -1], [1, -1, -1], [-1, -1, 1], [-1, -1, 1], [1, -1, -1], [1, -1, 1],
    ],
    colors: [
        // front - blue
        [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1],
        // right - red
        [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0],
        // back - yellow
        [1, 1, 0], [1, 1, 0], [1, 1, 0], [1, 1, 0], [1, 1, 0], [1, 1, 0],
        // left - aqua
        [0, 1, 1], [0, 1, 1], [0, 1, 1], [0, 1, 1], [0, 1, 1], [0, 1, 1],
        // top - green
        [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0],
        // bottom - fuchsia
        [1, 0, 1], [1, 0, 1], [1, 0, 1], [1, 0, 1], [1, 0, 1], [1, 0, 1],
    ],
    uvs: [
        // front
        [0, 0], [1, 0], [0, 1], [0, 1], [1, 0], [1, 1],
        // right
        [0, 0], [1, 0], [0, 1], [0, 1], [1, 0], [1, 1],
        // back
        [0, 0], [1, 0], [0, 1], [0, 1], [1, 0], [1, 1],
        // left
        [0, 0], [1, 0], [0, 1], [0, 1], [1, 0], [1, 1],
        // top
        [0, 0], [1, 0], [0, 1], [0, 1], [1, 0], [1, 1],
        // bottom
        [0, 0], [1, 0], [0, 1], [0, 1], [1, 0], [1, 1],
    ],
    normals: [
        // front
        [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1],
        // right
        [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0],
        // back
        [0, 0, -1], [0, 0, -1], [0, 0, -1], [0, 0, -1], [0, 0, -1], [0, 0, -1],
        // left
        [-1, 0, 0], [-1, 0, 0], [-1, 0, 0], [-1, 0, 0], [-1, 0, 0], [-1, 0, 0],
        // top
        [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0],
        // bottom
        [0, -1, 0], [0, -1, 0], [0, -1, 0], [0, -1, 0], [0, -1, 0], [0, -1, 0],
    ],
};

pub struct CubeUvData {
    pub positions: [[i8; 3]; 36],
    pub uvs: [[f32; 2]; 36],
    pub normals: [[i8; 3]; 36],
}


pub const MULTI_TEXTURE_CUBE: CubeUvData = CubeUvData {
    positions: FACE_COLORS_CUBE.positions,
    normals: FACE_COLORS_CUBE.normals,
    uvs: [
        //front
        [0.0, 1.0 / 2.0], [1.0 / 3.0, 1.0 / 2.0], [0.0, 1.0], [0.0, 1.0], [1.0 / 3.0, 1.0 / 2.0], [1.0 / 3.0, 1.0],
        //right
        [1.0 / 3.0, 1.0 / 2.0], [2.0 / 3.0, 1.0 / 2.0], [1.0 / 3.0, 1.0], [1.0 / 3.0, 1.0], [2.0 / 3.0, 1.0 / 2.0], [2.0 / 3.0, 1.0],
        //back
        [2.0 / 3.0, 1.0 / 2.0], [1.0, 1.0 / 2.0], [2.0 / 3.0, 1.0], [2.0 / 3.0, 1.0], [1.0, 1.0 / 2.0], [1.0, 1.0],
        //left
        [0.0, 0.0], [0.0, 1.0 / 2.0], [1.0 / 3.0, 0.0], [1.0 / 3.0, 0.0], [0.0, 1.0 / 2.0], [1.0 / 3.0, 1.0 / 2.0],
        //top
        [1.0 / 3.0, 0.0], [2.0 / 3.0, 0.0], [1.0 / 3.0, 1.0 / 2.0], [1.0 / 3.0, 1.0 / 2.0], [2.0 / 3.0, 0.0], [2.0 / 3.0, 1.0 / 2.0],
        //bottom
        [2.0 / 3.0, 1.0 / 2.0], [1.0, 1.0 / 2.0], [2.0 / 3.0, 0.0], [2.0 / 3.0, 0.0], [1.0, 1.0 / 2.0], [1.0, 0.0],
    ],
};

pub struct CubeIndexData {
    pub positions: [[i8; 3]; 8],
    pub colors: [[i8; 3]; 8],
    pub indices: [u16; 36],
}

pub const CUBE_INDEX_DATA: CubeIndexData = {
    CubeIndexData {
        positions: [
            [-1, -1, 1],  // vertex a
            [1, -1, 1],   // vertex b
            [1, 1, 1],    // vertex c
            [-1, 1, 1],   // vertex d
            [-1, -1, -1], // vertex e
            [1, -1, -1],  // vertex f
            [1, 1, -1],   // vertex g
            [-1, 1, -1],  // vertex h
        ],
        colors: [
            [0, 0, 1], // vertex a
            [1, 0, 1], // vertex b
            [1, 1, 1], // vertex c
            [0, 1, 1], // vertex d
            [0, 0, 0], // vertex e
            [1, 0, 0], // vertex f
            [1, 1, 0], // vertex g
            [0, 1, 0], // vertex h
        ],
        indices: [
            0, 1, 2, 2, 3, 0, // front
            1, 5, 6, 6, 2, 1, // right
            4, 7, 6, 6, 5, 4, // back
            0, 3, 7, 7, 4, 0, // left
            3, 2, 6, 6, 7, 3, // top
            0, 4, 5, 5, 1, 0, // bottom
        ],
    }
};

pub fn cylinder_position<T: Into<Deg<f32>>>(r: f32, y: f32, theta: T) -> Point3<f32> {
    let (sin_theta, cos_theta) = theta.into().sin_cos();
    point3(r * cos_theta, y, -r * sin_theta)
}

#[must_use]
pub fn torus_position(r_torus: f32, r_tube: f32, u: Deg<f32>, v: Deg<f32>) -> Point3<f32> {
    let (sin_v, cos_v) = v.sin_cos();
    let (sin_u, cos_u) = u.sin_cos();
    let r = r_torus + r_tube * cos_v;
    point3(r * cos_u, r_tube * sin_v, -r * sin_u)
}

type SphereVertexF<'a, V> = &'a dyn Fn(Point3<f32>, Vector3<f32>, Point2<f32>) -> V;
type ItemF<'a, V, R> = &'a dyn Fn([V; 4]) -> Vec<R>;

#[must_use]
pub fn sphere_vertex<V: Copy>(vertex: SphereVertexF<V>, center: Point3<f32>, r: f32, uv: Point2<f32>) -> V {
    let (sin_lon, cos_lon) = (Deg::full_turn() * uv.x).sin_cos();
    let (sin_lat, cos_lat) = (Deg::full_turn() * uv.y / 2.0).sin_cos();
    let normal = vec3(sin_lat * cos_lon, cos_lat, -sin_lat * sin_lon);
    vertex(center + r * normal, normal, uv)
}

#[must_use]
pub fn sphere_vertices<V: Copy, R>(
    center: Point3<f32>,
    r: f32,
    u: usize,
    v: usize,
    vertex_f: SphereVertexF<V>,
    item_f: ItemF<V, R>
) -> Vec<R> {
    let du = 1.0 / u as f32;
    let dv = 1.0 / v as f32;

    (0..u).flat_map(|u_i| (0..v).flat_map(move |v_i| {
            let u_v = du * u_i as f32;
            let v_v = dv * v_i as f32;
            let u_v1 = du * (u_i + 1) as f32;
            let v_v1 = dv * (v_i + 1) as f32;
            let p0 = sphere_vertex(vertex_f, center, r, point2(u_v, v_v));
            let p1 = sphere_vertex(vertex_f, center, r, point2(u_v1, v_v));
            let p2 = sphere_vertex(vertex_f, center, r, point2(u_v1, v_v1));
            let p3 = sphere_vertex(vertex_f, center, r, point2(u_v, v_v1));

            item_f([p0, p1, p2, p3])
    })).collect()
}

#[must_use]
pub fn sphere_faces<V: Copy>(center: Point3<f32>, r: f32, u: usize, v: usize, vertex_f: SphereVertexF<V>) -> Vec<V> {
    sphere_vertices(center, r, u, v, vertex_f, &|[p0, p1, p2, p3]| vec![[p0, p3, p1], [p1, p3, p2]])
        .into_iter().flatten().collect()
}

#[must_use]
pub fn sphere_edges<V: Copy>(center: Point3<f32>, r: f32, u: usize, v: usize, vertex_f: SphereVertexF<V>) -> Mesh<V> {
    Mesh::from(sphere_vertices(center, r, u, v, vertex_f, &|[v0, v1, _, v3]| vec![(v0, v1), (v0, v3)]).into_iter())
}
