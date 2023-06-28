#![allow(clippy::module_name_repetitions)]

use bytemuck::{Pod, Zeroable};
use cgmath::{InnerSpace, Matrix4, Point2, Point3, vec4, Vector3, Vector4};

use webgpu_book::VertexBufferInfo;

// Vertex with position only

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 4],
}

impl Vertex {
    #[allow(dead_code)]
    pub fn new<P: Into<Point3<f32>>>(position: P) -> Self {
        Self { position: position.into().to_homogeneous().into() }
    }
}

impl VertexBufferInfo for Vertex {
    const ATTRIBUTES: &'static [wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![0=>Float32x4];
}

impl From<Vertex> for VertexN {
    fn from(value: Vertex) -> Self {
        Self { position: value.position, normal: VertexN::FAKE_NORMAL.into() }
    }
}


// Vertex with position and color

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct VertexC {
    position: [f32; 4],
    color: [f32; 4],
}

#[allow(dead_code)]
impl VertexC {
    pub fn new<P: Into<Point3<f32>>, C: Into<Point3<f32>>>(position: P, color: C) -> Self {
        Self {
            position: position.into().to_homogeneous().into(),
            color: color.into().to_homogeneous().into(),
        }
    }
}

impl VertexBufferInfo for VertexC {
    const ATTRIBUTES: &'static [wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![0=>Float32x4, 1=>Float32x4];
}

impl From<VertexC> for VertexN {
    fn from(value: VertexC) -> Self {
        Self { position: value.position, normal: VertexN::FAKE_NORMAL.into() }
    }
}


// Vertex with position and normal

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct VertexN {
    pub position: [f32; 4],
    pub normal: [f32; 4],
}

#[allow(dead_code)]
impl VertexN {
    const FAKE_NORMAL: Vector4<f32> = vec4(0.0, 0.0, 0.0, 0.0);

    pub fn new<P: Into<Point3<f32>>, N: Into<Vector3<f32>>>(position: P, normal: N) -> Self {
        Self {
            position: position.into().to_homogeneous().into(),
            normal: normal.into().normalize().extend(0.0).into(),
        }
    }

    pub(crate) fn normal_vertex(&self, normal_len: f32) -> Self {
        Self {
            position: (Vector4::from(self.position) + Vector4::from(self.normal) * normal_len).into(),
            normal: Self::FAKE_NORMAL.into()
        }
    }
}

impl VertexBufferInfo for VertexN {
    const ATTRIBUTES: &'static [wgpu::VertexAttribute] = &wgpu::vertex_attr_array![0=>Float32x4, 1=>Float32x4];
}

// Vertex with position, normal, and color

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct VertexNC {
    pub position: [f32; 4],
    pub normal: [f32; 4],
    pub color: [f32; 4],
}

impl VertexNC {
    #[allow(dead_code)]
    pub fn new<P, N, C>(position: P, normal: N, color: C)
        -> Self where P: Into<Point3<f32>>, N: Into<Vector3<f32>>, C: Into<Point3<f32>>
    {
        Self {
            position: position.into().to_homogeneous().into(),
            normal: normal.into().normalize().extend(0.0).into(),
            color: color.into().to_homogeneous().into(),
        }
    }
}

impl From<VertexNC> for VertexN {
    fn from(value: VertexNC) -> Self {
        VertexN { position: value.position, normal: value.normal }
    }
}

impl VertexBufferInfo for VertexNC {
    const ATTRIBUTES: &'static [wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![0=>Float32x4, 1=>Float32x4, 2=>Float32x4];
}


// Vertex with position, normal, and texture coordinates

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct VertexNT {
    pub position: [f32; 4],
    pub normal: [f32; 4],
    pub uv: [f32; 2],
}

impl VertexNT {
    #[allow(dead_code)]
    pub fn new<P, N, UV>(position: P, normal: N, uv: UV)
        -> Self where P: Into<Point3<f32>>, N: Into<Vector3<f32>>, UV: Into<Point2<f32>>
    {
        Self {
            position: position.into().to_homogeneous().into(),
            normal: normal.into().normalize().extend(0.0).into(),
            uv: uv.into().into(),
        }
    }
}

impl VertexBufferInfo for VertexNT {
    const ATTRIBUTES: &'static [wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![0=>Float32x4, 1=>Float32x4, 2=>Float32x2];
}

impl From<VertexNT> for VertexN {
    fn from(value: VertexNT) -> Self {
        Self { position: value.position, normal: value.normal }
    }
}


// Vertex with position, normal, texture coordinates, and color

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct VertexNCT {
    pub position: [f32; 4],
    pub normal: [f32; 4],
    pub color: [f32; 4],
    pub uv: [f32; 2],
}

impl VertexNCT {
    #[allow(dead_code)]
    pub fn new<P, N, C, UV>(position: P, normal: N, color: C, uv: UV)
        -> Self where P: Into<Point3<f32>>, N: Into<Vector3<f32>>, C: Into<Point3<f32>>, UV: Into<Point2<f32>>
    {
        Self {
            position: position.into().to_homogeneous().into(),
            normal: normal.into().normalize().extend(0.0).into(),
            color: color.into().to_homogeneous().into(),
            uv: uv.into().into(),
        }
    }

    #[must_use] pub fn transform(&self, transform: Matrix4<f32>) -> Self {
        VertexNCT {
            position: (transform * <Vector4<f32>>::from(self.position)).into(),
            ..*self
        }
    }
}

impl VertexBufferInfo for VertexNCT {
    const ATTRIBUTES: &'static [wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![0=>Float32x4, 1=>Float32x4, 2=>Float32x4, 3=>Float32x2];
}

impl From<VertexNCT> for VertexNT {
    fn from(value: VertexNCT) -> Self {
        Self { position: value.position, normal: value.normal, uv: value.uv }
    }
}

impl From<VertexNCT> for VertexNC {
    fn from(value: VertexNCT) -> Self {
        Self { position: value.position, normal: value.normal, color: value.color }
    }
}

impl From<VertexNCT> for VertexN {
    fn from(value: VertexNCT) -> Self {
        Self { position: value.position, normal: value.normal }
    }
}
