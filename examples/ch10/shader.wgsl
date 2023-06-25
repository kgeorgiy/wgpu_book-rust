struct VertexUniforms {
    model: mat4x4<f32>,
    model_it: mat4x4<f32>,
    view_project: mat4x4<f32>,
};
@group(0) @binding(0) var<uniform> vertex_u: VertexUniforms;

struct Input {
    @location(0) pos: vec4<f32>,
    @location(1) normal: vec4<f32>,
    @location(2) uv: vec2<f32>,
};

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) v_position: vec4<f32>,
    @location(1) v_normal: vec4<f32>,
    @location(2) v_uv: vec2<f32>,
};

@vertex
fn vs_main(in: Input) -> Output {
    let position: vec4<f32> = vertex_u.model * in.pos;

    var output: Output;
    output.position = vertex_u.view_project * position;
    output.v_position = position;
    output.v_normal = vertex_u.model_it * in.normal;
    output.v_uv = in.uv;
    return output;
}


struct FragmentUniforms {
    light_position: vec4<f32>,
    eye_position: vec4<f32>,
};
@group(0) @binding(1) var<uniform> fragment_u: FragmentUniforms;

struct LightUniforms {
    specular_color: vec4<f32>,
    ambient_intensity: f32,
    diffuse_intensity: f32,
    specular_intensity: f32,
    specular_shininess: f32,
    is_two_side: i32,
};
@group(0) @binding(2) var<uniform> light_u: LightUniforms;

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

    var diffuse: f32 = diffuse(dotNL);
    var specular: f32 = specular(dotNH);

    if (light_u.is_two_side != 0) {
        diffuse += 0.5 * diffuse(-dotNL);
        specular += 0.5 * specular(-dotNH);
    }

    let ambient = light_u.ambient_intensity;
    return vec4(color * (ambient + diffuse) + light_u.specular_color.xyz * specular, 1.0);
}

@group(1) @binding(0) var texture_data: texture_2d<f32>;
@group(1) @binding(1) var texture_sampler: sampler;

@fragment
fn fs_main(in: Output) -> @location(0) vec4<f32> {
    return color(in.v_position, in.v_normal, textureSample(texture_data, texture_sampler, in.v_uv).rgb);
}
