struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) deltas: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) radius: f32,
    @location(3) center: vec4<f32>,
}

@vertex
fn vs_main(in: Sphere) -> Output {
    let model_center = model_u.points * in.center;
    let center = camera_u.view * model_center;
    var deltas = array<vec2<f32>, 4>(
        vec2f( 1.0,  1.0),
        vec2f( 1.0, -1.0),
        vec2f(-1.0, -1.0),
        vec2f(-1.0,  1.0),
    );
    let position = vec4f(deltas[in.index] * in.radius, 0.0, 0.0);

    var output: Output;

    output.position = camera_u.project * (center + position);
    output.deltas = deltas[in.index];
    output.color = in.color;
    output.radius = in.radius;
    output.center = model_center;
    return output;
}

struct FragmentOutput {
    @location(0) color: vec4<f32>,
    @builtin(sample_mask) mask: u32,
    @builtin(frag_depth) depth: f32,
 }

@fragment
fn fs_main(in: Output) -> FragmentOutput {
    let r: f32 = dot(in.deltas, in.deltas);

    var out: FragmentOutput;

    if (r <= 1.0) {
        let z = sqrt(1.0 - r);
        let normal = vec4(in.deltas, z, 0.0);
        let position = in.center + normal * in.radius;

        out.mask = ~0u;
        out.color = color(position, transpose(camera_u.view) * normal, in.color.xyz);
//        out.color = in.color;
        out.depth = (camera_u.project * position).z;
    }
    return out;
}
