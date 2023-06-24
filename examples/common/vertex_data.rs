#![allow(dead_code)]

use cgmath::{Angle, Deg, Point3, point3};

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

pub fn sphere_position(r: f32, theta: Deg<f32>, phi: Deg<f32>) -> Point3<f32> {
    let (sin_theta, cos_theta) = theta.sin_cos();
    let (sin_phi, cos_phi) = phi.sin_cos();
    point3(r * sin_theta * cos_phi, r * cos_theta, -r * sin_theta * sin_phi)
}

pub fn cylinder_position<T: Into<Deg<f32>>>(r: f32, y: f32, theta: T) -> Point3<f32> {
    let (sin_theta, cos_theta) = theta.into().sin_cos();
    point3(r * cos_theta, y, -r * sin_theta)
}

pub fn torus_position(r_torus: f32, r_tube: f32, u: Deg<f32>, v: Deg<f32>) -> Point3<f32> {
    let (sin_v, cos_v) = v.sin_cos();
    let (sin_u, cos_u) = u.sin_cos();
    let r = r_torus + r_tube * cos_v;
    point3(r * cos_u, r_tube * sin_v, -r * sin_u)
}
