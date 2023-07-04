struct Output {
    @builtin(position) proj_position: vec4<f32>,
    @location(0) deltas: vec2<f32>,
    @interpolate(linear) @location(1) pos: vec4f,
    @interpolate(flat) @location(2) color: vec4<f32>,
    @interpolate(flat) @location(3) scene_center: vec4<f32>,
    @interpolate(flat) @location(4) PVMTi_0: vec4f,
    @interpolate(flat) @location(5) PVMTi_1: vec4f,
    @interpolate(flat) @location(6) PVMTi_2: vec4f,
    @interpolate(flat) @location(7) PVMTi_3: vec4f,
}

fn sphereD(r1: vec4f, r2: vec4f) -> f32 {
    return dot(r1.xyz, r2.xyz) - r1.w * r2.w;
}

fn find_box(a: f32, mb2: f32, r: vec4f) -> vec2f {
    let disc = sqrt(mb2 * mb2 - a * sphereD(r, r));
    return vec2f(-disc / a, mb2 / a);
}

@vertex
fn vs_main(in: Sphere) -> Output {
    let T = transpose(mat4x4f(
        1.0, 0.0, 0.0, in.center.x / in.radius,
        0.0, 1.0, 0.0, in.center.y / in.radius,
        0.0, 0.0, 1.0, in.center.z / in.radius,
        0.0, 0.0, 0.0, 1.0         / in.radius,
    ));
    let Ti = transpose(mat4x4f(
        1.0, 0.0, 0.0, -in.center.x,
        0.0, 1.0, 0.0, -in.center.y,
        0.0, 0.0, 1.0, -in.center.z,
        0.0, 0.0, 0.0, in.radius,
    ));

    var out: Output;
    {
        let PVMTt = transpose(camera_u.project * camera_u.view * model_u.points * T);

        let rx = PVMTt[0];
        let ry = PVMTt[1];
        let rw = PVMTt[3];
        let wDw = sphereD(rw, rw);
        let xDw = sphereD(rx, rw);
        let yDw = sphereD(ry, rw);

        var deltas = array<vec2<f32>, 4>(
            vec2f( 1.0,  1.0),
            vec2f( 1.0, -1.0),
            vec2f(-1.0, -1.0),
            vec2f(-1.0,  1.0),
        );
        let ds = deltas[in.index];


        out.proj_position = vec4f(
            dot(vec2f(ds.x, 1.0), find_box(wDw, xDw, rx)),
            dot(vec2f(ds.y, 1.0), find_box(wDw, yDw, ry)),
            0.0,
            1.0,
        );
        out.pos = out.proj_position;
    }

    let PVMTi = Ti * transpose(model_u.normals) * camera_u.project_view_inverse;
    out.PVMTi_0 = PVMTi[0];
    out.PVMTi_1 = PVMTi[1];
    out.PVMTi_2 = PVMTi[2];
    out.PVMTi_3 = PVMTi[3];
    out.scene_center = model_u.points * in.center;

    out.color = in.color; // vec4(ds / 2.0 + vec2f(0.5, 0.5), 0.0, 1.0);
    return out;
}

struct FragmentOutput {
    @location(0) color: vec4<f32>,
    @builtin(sample_mask) mask: u32,
    @builtin(frag_depth) depth: f32,
 }

@fragment
fn fs_main(in: Output) -> FragmentOutput {
    var out: FragmentOutput;

    let PVMTi = mat4x4f(in.PVMTi_0, in.PVMTi_1, in.PVMTi_2, in.PVMTi_3);
    let p = PVMTi * vec4f(in.pos.xy, 0.0, 1.0);
    let c3 = PVMTi[2];

    let a = sphereD(c3, c3);
    let b2 = sphereD(p, c3);
    let c = sphereD(p, p);

    let disc2 = b2 * b2 - a * c;

    if (disc2 < 0.01) {
        return out;
    }

    let z = -b2 / a - abs(sqrt(disc2) / a);

    out.mask = ~0u;
    out.color = in.color * (z + 1.0) / 2.0;
    out.depth = z;

    let scene_position_raw = camera_u.project_view_inverse * vec4(in.pos.xy, z, 1.0);
    let scene_position = vec4(scene_position_raw.xyz / scene_position_raw.w, 1.0);
    let scene_normal = vec4(normalize((scene_position - in.scene_center).xyz), 0.0);
//    out.color = vec4(scene_normal.xyz / 2.0 + 0.5, 1.0);

    out.color = color(scene_position, scene_normal, in.color.rgb);
    return out;
}
