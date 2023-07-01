#![allow(dead_code)]

use cgmath::{Angle, Deg, Point2, point2, Point3, point3, vec3, Vector3};
use super::surface_data::{Quads, Triangles};
use super::{VertexN, VertexNT};

#[must_use]
pub fn i8_as_f32<const R: usize, const Q: usize, const C: usize>(values: [[[i8; C]; Q]; R]) -> [[[f32; C]; Q]; R] {
    values.map(|quad| quad.map(|vertex| vertex.map(f32::from)))
}

pub struct CubeFaceData {
    pub positions: [[[i8; 3]; 4]; 6],
    pub colors: [[[i8; 3]; 4]; 6],
    pub uvs: [[[i8; 2]; 4]; 6],
    pub normals: [[[i8; 3]; 4]; 6],
}

pub const FACE_COLORS_CUBE: CubeFaceData = CubeFaceData {
    positions: [
        // front (0, 0, 1)
        [[1, -1, 1], /**/[-1, -1, 1], [-1, 1, 1], [1, 1, 1]],
        // right (1, 0, 0)
        [[1, -1, -1], /**/[1, -1, 1], [1, 1, 1], [1, 1, -1]],
        // back (0, 0, -1)
        [[-1, -1, -1], /**/[1, -1, -1], [1, 1, -1], [-1, 1, -1]],
        // left (-1, 0, 0)
        [[-1, -1, 1], /**/[-1, -1, -1], [-1, 1, -1], [-1, 1, 1]],
        // top (0, 1, 0)
        [[1, 1, 1], /**/[-1, 1, 1], [-1, 1, -1], [1, 1, -1]],
        // bottom (0, -1, 0)
        [[1, -1, -1], /**/[-1, -1, -1], [-1, -1, 1], [1, -1, 1]],
    ],
    colors: [
        // front - blue
        [[0, 0, 1], /**/[0, 0, 1], [0, 0, 1], [0, 0, 1]],
        // right - red
        [[1, 0, 0], /**/[1, 0, 0], [1, 0, 0], [1, 0, 0]],
        // back - yellow
        [[1, 1, 0], /**/[1, 1, 0], [1, 1, 0], [1, 1, 0]],
        // left - aqua
        [[0, 1, 1], /**/[0, 1, 1], [0, 1, 1], [0, 1, 1]],
        // top - green
        [[0, 1, 0], /**/[0, 1, 0], [0, 1, 0], [0, 1, 0]],
        // bottom - fuchsia
        [[1, 0, 1], /**/[1, 0, 1], [1, 0, 1], [1, 0, 1]],
    ],
    uvs: [
        // front
        [[1, 0], /**/[0, 0], [0, 1], [1, 1]],
        // right
        [[1, 0], /**/[0, 0], [0, 1], [1, 1]],
        // back
        [[1, 0], /**/[0, 0], [0, 1], [1, 1]],
        // left
        [[1, 0], /**/[0, 0], [0, 1], [1, 1]],
        // top
        [[1, 0], /**/[0, 0], [0, 1], [1, 1]],
        // bottom
        [[1, 0], /**/[0, 0], [0, 1], [1, 1]],
    ],
    normals: [
        // front
        [[0, 0, 1], /**/[0, 0, 1], [0, 0, 1], [0, 0, 1]],
        // right
        [[1, 0, 0], /**/[1, 0, 0], [1, 0, 0], [1, 0, 0]],
        // back
        [[0, 0, -1], /**/[0, 0, -1], [0, 0, -1], [0, 0, -1]],
        // left
        [[-1, 0, 0], /**/[-1, 0, 0], [-1, 0, 0], [-1, 0, 0]],
        // top
        [[0, 1, 0], /**/[0, 1, 0], [0, 1, 0], [0, 1, 0]],
        // bottom
        [[0, -1, 0], /**/[0, -1, 0], [0, -1, 0], [0, -1, 0]],
    ],
};

pub struct CubeUvData {
    pub positions: [[[i8; 3]; 4]; 6],
    pub uvs: [[[f32; 2]; 4]; 6],
    pub normals: [[[i8; 3]; 4]; 6],
}


pub const MULTI_TEXTURE_CUBE: CubeUvData = CubeUvData {
    positions: FACE_COLORS_CUBE.positions,
    normals: FACE_COLORS_CUBE.normals,
    uvs: [
        //front
        [[0.0, 1.0], /**/[0.0, 1.0 / 2.0], [1.0 / 3.0, 1.0 / 2.0], [1.0 / 3.0, 1.0]],
        //right
        [[1.0 / 3.0, 1.0], /**/[1.0 / 3.0, 1.0 / 2.0], [2.0 / 3.0, 1.0 / 2.0], [2.0 / 3.0, 1.0]],
        //back
        [[2.0 / 3.0, 1.0], /**/[2.0 / 3.0, 1.0 / 2.0], [1.0, 1.0 / 2.0], [1.0, 1.0]],
        //left
        [[1.0 / 3.0, 0.0], /**/[0.0, 0.0], [0.0, 1.0 / 2.0], [1.0 / 3.0, 1.0 / 2.0]],
        //top
        [[1.0 / 3.0, 1.0 / 2.0], /**/[1.0 / 3.0, 0.0], [2.0 / 3.0, 0.0], [2.0 / 3.0, 1.0 / 2.0]],
        //bottom
        [[2.0 / 3.0, 0.0], /**/[2.0 / 3.0, 1.0 / 2.0], [1.0, 1.0 / 2.0], [1.0, 0.0]],
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

//
// Cylinder

pub struct Cylinder;

impl Cylinder {
    pub fn quads(rin: f32, rout: f32, height: f32, n: usize, ul: f32, vl: f32) -> Quads<VertexNT> {
        let h = height / 2.0;
        let d_theta = Deg(360.0 / n as f32);

        let top = h;
        let bot = -h;
        let up = vec3(0.0, 1.0, 0.0);
        let du = ul / 360.0;
        let dl = vl / h / 2.0;
        let face_params = (ul, rin, vl / height);

        (0..n).flat_map(|i| {
            let theta_1 = d_theta * i as f32;
            let theta_2 = d_theta * (i + 1) as f32;

            [
                [ // top face
                    Self::face(rout, top, theta_2, up, face_params),
                    Self::face(rout, top, theta_1, up, face_params),
                    Self::face(rin, top, theta_1, up, face_params),
                    Self::face(rin, top, theta_2, up, face_params),
                ],
                [ // bottom face
                    Self::face(rin, bot, theta_1, -up, face_params),
                    Self::face(rout, bot, theta_1, -up, face_params),
                    Self::face(rout, bot, theta_2, -up, face_params),
                    Self::face(rin, bot, theta_2, -up, face_params),
                ],
                [ // outer face
                    Self::side(bot, theta_1, rout, 1.0, h, du, dl),
                    Self::side(top, theta_1, rout, 1.0, h, du, dl),
                    Self::side(top, theta_2, rout, 1.0, h, du, dl),
                    Self::side(bot, theta_2, rout, 1.0, h, du, dl),
                ],
                [ // inner face
                    Self::side(top, theta_1, rin, -1.0, h, du, dl),
                    Self::side(bot, theta_1, rin, -1.0, h, du, dl),
                    Self::side(bot, theta_2, rin, -1.0, h, du, dl),
                    Self::side(top, theta_2, rin, -1.0, h, du, dl),
                ],
            ]
        }).into()
    }

    fn face(r: f32, h: f32, theta: Deg<f32>, normal: Vector3<f32>, (ul, rin, vc): (f32, f32, f32)) -> VertexNT {
        let u = ul * theta.0 / 360.0;
        let v = vc * (r - rin);
        VertexNT::new(Self::position(r, h, theta), normal, (u, v))
    }

    fn side(y: f32, theta: Deg<f32>, r: f32, dn: f32, h: f32, du: f32, dv: f32) -> VertexNT {
        let p = Self::position(r, y, theta);
        VertexNT::new(p, (p - point3(0.0, y, 0.0)) * dn, (du * theta.0, (y + h) * dv))
    }

    pub fn position<T: Into<Deg<f32>>>(r: f32, y: f32, theta: T) -> Point3<f32> {
        let (sin_theta, cos_theta) = theta.into().sin_cos();
        point3(r * cos_theta, y, -r * sin_theta)
    }
}


pub struct Torus;

impl Torus {
    pub fn quads(r_torus: f32, r_tube: f32, n_torus: usize, n_tube: usize) -> Quads<VertexN> {
        let d_u = Deg::full_turn() / n_torus as f32;
        let d_v = Deg::full_turn() / n_tube as f32;

        (0..n_torus).flat_map(|i| (0..n_tube).map(move |j| {
            let u = d_u * i as f32;
            let v = d_v * j as f32;
            let u1 = d_u * (i as f32 + 1.0);
            let v1 = d_v * (j as f32 + 1.0);

            [
                Self::vertex(r_torus, r_tube, u, v),
                Self::vertex(r_torus, r_tube, u1, v),
                Self::vertex(r_torus, r_tube, u1, v1),
                Self::vertex(r_torus, r_tube, u, v1)
            ]
        })).into()
    }

    fn vertex(r_torus: f32, r_tube: f32, u: Deg<f32>, v: Deg<f32>) -> VertexN {
        let position = Self::position(r_torus, r_tube, u, v);
        let center = Self::position(r_torus, 0.0, u, v);
        VertexN::new(position, position - center)
    }

    fn position(r_torus: f32, r_tube: f32, u: Deg<f32>, v: Deg<f32>) -> Point3<f32> {
        let (sin_v, cos_v) = v.sin_cos();
        let (sin_u, cos_u) = u.sin_cos();
        let r = r_torus + r_tube * cos_v;
        point3(r * cos_u, r_tube * sin_v, -r * sin_u)
    }
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

pub fn sphere_quads<V: Copy>(
    center: Point3<f32>,
    r: f32,
    u: usize,
    v: usize,
    vertex_f: SphereVertexF<V>,
) -> Quads<V> {
    let du = 1.0 / u as f32;
    let dv = 1.0 / v as f32;

    (0..u).flat_map(|u_i| (0..v).map(move |v_i| {
            let u_v = du * u_i as f32;
            let v_v = dv * v_i as f32;
            let u_v1 = du * (u_i + 1) as f32;
            let v_v1 = dv * (v_i + 1) as f32;
            let p0 = sphere_vertex(vertex_f, center, r, point2(u_v, v_v));
            let p1 = sphere_vertex(vertex_f, center, r, point2(u_v1, v_v));
            let p2 = sphere_vertex(vertex_f, center, r, point2(u_v1, v_v1));
            let p3 = sphere_vertex(vertex_f, center, r, point2(u_v, v_v1));

            [p0, p1, p2, p3]
    })).into()
}

//
// Cone

pub struct Cone;

impl Cone {
    const ORIGIN: Point3<f32> = point3(0.0, 0.0, 0.0);

    pub fn triangles(r_top: f32, r_bottom: f32, height: f32, n: usize) -> Triangles<VertexN> {
        let h = height / 2.0;
        let d_theta = Deg(360.0 / n as f32);

        let up = vec3(0.0, 1.0, 0.0);

        (0..n).flat_map(|i| {
            let theta = d_theta * i as f32;
            let theta_1 = d_theta * (i + 1) as f32;

            let top_out = Cylinder::position(r_top, h, theta);
            let bot_out = Cylinder::position(r_bottom, -h, theta);
            let bot_cen = Cylinder::position(0.0, -h, theta);
            let top_cen = Cylinder::position(0.0, h, theta);
            let top_out_1 = Cylinder::position(r_top, h, theta_1);
            let bot_out_1 = Cylinder::position(r_bottom, -h, theta_1);

            [
                [ // top face
                    VertexN::new(top_out, up),
                    VertexN::new(top_out_1, up),
                    VertexN::new(top_cen, up),
                ],
                [ // bottom face
                    VertexN::new(bot_out, -up),
                    VertexN::new(bot_cen, -up),
                    VertexN::new(bot_out_1, -up),
                ],
                [ // Outer face 1
                    Self::outer(top_out, bot_out),
                    Self::outer(bot_out, top_out),
                    Self::outer(bot_out_1, top_out_1),
                ],
                [ // Outer face 2

                    Self::outer(bot_out_1, top_out_1),
                    Self::outer(top_out_1, bot_out_1),
                    Self::outer(top_out, bot_out),
                ]
            ]
        }).into()
    }

    fn outer(p: Point3<f32>, other: Point3<f32>) -> VertexN {
        let dp = other - p;
        VertexN::new(p, (Self::ORIGIN - p).cross(dp).cross(dp))
    }
}


pub fn sphere_triangles<V: Copy>(center: Point3<f32>, r: f32, u: usize, v: usize, vertex_f: SphereVertexF<V>) -> Triangles<V> {
    sphere_quads(center, r, u, v, vertex_f).triangles()
}
