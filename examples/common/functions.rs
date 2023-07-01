#![allow(dead_code)]

use core::f32::consts::PI;

use cgmath::{Point3, point3};

#[must_use] pub fn sinc(x: f32, y: f32) -> f32 {
    let r = (x * x + y * y).sqrt();
    if r == 0.0 { 1.0 } else { r.sin() / r }
}

#[must_use] pub fn peaks(x: f32, z: f32) -> f32 {
    3.0 * (1.0 - x) * (1.0 - x) * (-(x * x) - (z + 1.0) * (z + 1.0)).exp()
        - 10.0 * (x / 5.0 - x * x * x - z * z * z * z * z) * (-x * x - z * z).exp()
        - 1.0 / 3.0 * (-(x + 1.0) * (x + 1.0) - z * z).exp()
}

#[must_use] pub fn klein_bottle(u: f32, v: f32) -> Point3<f32> {
    let (sin_u, cos_u) = u.sin_cos();
    let (sin_v, cos_v) = v.sin_cos();
    let cos_sin = cos_v * sin_u;

    point3(
        2.0 / 15.0 * (3.0 + 5.0 * cos_u * sin_u) * sin_v,
        2.0 / 15.0 * cos_u * (
            3.0 * cos_v - 30.0 * sin_u
                + 90.0 * cos_u.powf(4.0) * sin_u
                - 60.0 * cos_u.powf(6.0) * sin_u
                + 5.0 * cos_u * cos_v * sin_u
        ),
        1.0 / 15.0 * sin_u * (
            3.0 * cos_v - 3.0 * cos_u.powf(2.0) * cos_v
                - 48.0 * cos_u.powf(4.0) * cos_v
                + 48.0 * cos_u.powf(6.0) * cos_v
                - 60.0 * sin_u
                + 5.0 * cos_u * cos_sin
                - 5.0 * cos_u.powf(3.0) * cos_sin
                - 80.0 * cos_u.powf(5.0) * cos_sin
                + 80.0 * cos_u.powf(7.0) * cos_sin
        ),
    )
}

#[must_use] pub fn wellenkugel(u: f32, v: f32) -> Point3<f32> {
    let (sin_v, cos_v) = v.sin_cos();
    let (sin_cos_u, cos_cos_u) = u.cos().sin_cos();
    point3(cos_cos_u * sin_v, sin_cos_u, cos_cos_u * cos_v) * u
}

#[must_use] pub fn breather(u: f32, v: f32) -> Point3<f32> {
    const A: f32 = 0.4; // where 0 < A < 1

    let aa1 = 1.0 - A * A;
    let aa1s = aa1.sqrt();
    let au_cosh = (A * u).cosh();
    let au_sinh = (A * u).sinh();

    let (av_sin, av_cos) = (aa1s * v).sin_cos();

    let de = A * (aa1 * au_cosh.powf(2.0) + (A * av_sin).powf(2.0));

    point3(
        -u * de / 2.0 + aa1 * au_cosh * au_sinh,
        aa1s * au_cosh * (-aa1s * v.cos() * av_cos - v.sin() * av_sin),
        aa1s * au_cosh * (-aa1s * v.sin() * av_cos + v.cos() * av_sin)
    ) * 2.0 / de
}

#[must_use] pub fn seashell(u: f32, v: f32) -> Point3<f32> {
    let v2_cos2 = (v / 2.0).cos().powf(2.0);
    let v_sin = v.sin();
    let u_exp = u.exp();

    let (sin_u, cos_u) = (u * 6.0 * PI).sin_cos();
    point3(
        2.0 * (u_exp - 1.0) * sin_u * v2_cos2,
        1.0 - (u * 2.0).exp() - v_sin + u_exp * v_sin,
        2.0 * (1.0 - u_exp) * cos_u * v2_cos2,
    )
}

#[must_use]
pub fn sievert_enneper(u: f32, v: f32) -> Point3<f32> {
    const A: f32 = 1.0;

    let pu = -u / (1.0 + A).sqrt() + (u.tan() * (1.0 + A).sqrt()).atan();
    let (sin_v, cos_v) = v.sin_cos();
    let (sin_u, cos_u) = u.sin_cos();

    let a_uv = 2.0 / (1.0 + A - A * (sin_v * cos_u).powf(2.0));
    let r_uv = a_uv * sin_v * ((1.0 + 1.0 / A) * (1.0 + A * sin_u * sin_u)).sqrt();
    let x = ((v / 2.0).tan().ln() + (1.0 + A) * a_uv * cos_v) / A.sqrt();

    point3(x, r_uv * pu.cos(), r_uv * pu.sin())
}

#[must_use]
pub fn sphere(u:f32, v:f32) -> Point3<f32> {
    let (sin_u, cos_u) = u.sin_cos();
    let (sin_v, cos_v) = v.sin_cos();
    point3(cos_v * cos_u, sin_v, cos_v * sin_u)
}

#[must_use]
pub fn torus(u: f32, v: f32) -> Point3<f32> {
    let (sin_u, cos_u) = u.sin_cos();
    let (sin_v, cos_v) = v.sin_cos();
    point3((1.0 + 0.3 * cos_v) * cos_u, 0.3 * sin_v, (1.0 + 0.3 * cos_v) * sin_u)
}