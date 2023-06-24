struct VertexUniforms {
    model: mat4x4<f32>,
    model_it: mat4x4<f32>,
    view_project: mat4x4<f32>,
};
@binding(0) @group(0) var<uniform> vertex_u: VertexUniforms;

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) v_position: vec4<f32>,
    @location(1) v_normal: vec4<f32>,
};

@vertex
fn vs_main(@location(0) pos: vec4<f32>, @location(1) normal: vec4<f32>) -> Output {
    var output: Output;
    let position: vec4<f32> = vertex_u.model * pos;
    output.position = vertex_u.view_project * position;
    output.v_position = position;
    output.v_normal = vertex_u.model_it * normal;
    return output;
}

struct FragmentUniforms {
    light_position: vec4<f32>,
    eye_position: vec4<f32>,
};
@binding(1) @group(0) var<uniform> fragment_u: FragmentUniforms;

struct LightUniforms {
    color: vec4<f32>,
    specular_color: vec4<f32>,
    ambient_intensity: f32,
    diffuse_intensity: f32,
    specular_intensity: f32,
    specular_shininess: f32,
};
@binding(2) @group(0) var<uniform> light_u: LightUniforms;

@fragment
fn fs_main(@location(0) v_position: vec4<f32>, @location(1) v_normal: vec4<f32>) -> @location(0) vec4<f32> {
    let N: vec3<f32> = normalize(v_normal.xyz);
    let L: vec3<f32> = normalize(fragment_u.light_position.xyz - v_position.xyz);
    let V: vec3<f32> = normalize(fragment_u.eye_position.xyz - v_position.xyz);
    let H: vec3<f32> = normalize(L + V);
    let diffuse: f32 = light_u.diffuse_intensity * max(dot(N, L), 0.0);
    let specular: f32 = light_u.specular_intensity * pow(max(dot(N, H), 0.0), light_u.specular_shininess);
    let ambient: f32 = light_u.ambient_intensity;
    return vec4(light_u.color.xyz * (ambient + diffuse) + light_u.specular_color.xyz * specular, 0.0);
}
