struct ModelUniforms {
    points: mat4x4<f32>,
    normals: mat4x4<f32>,
}
@group(0) @binding(0) var<uniform> model_u: ModelUniforms;

struct CameraUniforms {
    view: mat4x4<f32>,
    project: mat4x4<f32>,
}
@group(0) @binding(1) var<uniform> camera_u: CameraUniforms;

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) deltas: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) radius: f32,
    @location(3) center: vec4<f32>,
}


@vertex
fn vs_main(in: Sphere) -> Output {
    let center = camera_u.view * model_u.points * in.center;
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
    output.center = center;
    return output;
}

struct FragmentUniforms {
    light_position: vec4<f32>,
    eye_position: vec4<f32>,
}
@group(0) @binding(2) var<uniform> fragment_u: FragmentUniforms;

struct LightUniforms {
    specular_color: vec4<f32>,
    ambient_intensity: f32,
    diffuse_intensity: f32,
    specular_intensity: f32,
    specular_shininess: f32,
}
@group(0) @binding(3) var<uniform> light_u: LightUniforms;

fn diffuse(dotNL: f32) -> f32 {
    return light_u.diffuse_intensity * max(dotNL, 0.0);
}

fn specular(dotNH: f32) -> f32 {
    return light_u.specular_intensity * pow(max(dotNH, 0.0), light_u.specular_shininess);
}

fn color(position: vec4<f32>, normal: vec4<f32>, color: vec3<f32>) -> vec4<f32> {
    let N: vec3<f32> = normalize(normal.xyz);
    let L: vec3<f32> = normalize(fragment_u.light_position.xyz - position.xyz);
    let V: vec3<f32> = normalize(fragment_u.eye_position.xyz - position.xyz);
    let H: vec3<f32> = normalize(L + V);
    let dotNL = dot(N, L);
    let dotNH = dot(N, H);

    let diffuse: f32 = diffuse(dotNL);
    let specular: f32 = specular(dotNH);
    let ambient = light_u.ambient_intensity;

    return vec4(color * (ambient + diffuse) + light_u.specular_color.xyz * specular, 1.0);
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
        let position = in.center + vec4(normal) * in.radius;

        out.mask = ~0u;
        out.color = color(position, normal, in.color.xyz);
        out.color = in.color;
        out.depth = (camera_u.project * position).z;
    }
    return out;
}
